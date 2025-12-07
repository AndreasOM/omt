use std::collections::HashSet;
use std::path::Path;

use anyhow::{bail, Result};
use image::{GenericImageView, RgbaImage};

use super::oklab::{oklab_to_linear_rgb_unclamped, oklab_to_rgb, rgb_to_oklab};

struct UniqueColor {
	oklab:            [f32; 3],
	palette_position: usize,
}

struct PaletteLUT {
	grid:                       Vec<u32>,       // 256*256*256 = 16.7M entries (64MB)
	unique_colors:              Vec<UniqueColor>, // Keep for LUT building
	source_oklab_by_position: Vec<[f32; 3]>, // O(1) lookup by palette position
}

impl PaletteLUT {
	fn from_image(
		img: &image::DynamicImage,
		euclidean: bool,
		lightness_weight: f32,
	) -> Self {
		let rgba = img.to_rgba8();
		let raw = rgba.as_raw();

		println!("  Deduplicating palette colors...");
		// Step 1: Deduplicate palette - extract unique RGB colors
		let pixel_count = raw.len() / 4;
		let mut unique_colors = Vec::new();
		let mut seen = HashSet::new();

		for pos in 0..pixel_count {
			let idx = pos * 4;
			let rgb_u8 = [raw[idx], raw[idx + 1], raw[idx + 2]];

			if seen.insert(rgb_u8) {
				// First time seeing this color
				let rgb_f32 = [
					rgb_u8[0] as f32 / 255.0,
					rgb_u8[1] as f32 / 255.0,
					rgb_u8[2] as f32 / 255.0,
				];
				unique_colors.push(UniqueColor {
					oklab:            rgb_to_oklab(rgb_f32),
					palette_position: pos,
				});
			}
		}

		println!(
			"  Found {} unique colors (reduced from {} pixels)",
			unique_colors.len(),
			pixel_count
		);

		// Build position-indexed source palette for O(1) lookup
		let mut source_oklab_by_position = vec![[0.0f32; 3]; pixel_count];
		for unique_color in &unique_colors {
			source_oklab_by_position[unique_color.palette_position] = unique_color.oklab;
		}

		// Step 2: Build 256x256x256 lookup table
		println!("  Building 256^3 lookup table (16.7M entries)...");
		const GRID_SIZE: usize = 256 * 256 * 256;
		let mut grid = vec![0u32; GRID_SIZE];

		for r in 0..256 {
			if r % 32 == 0 {
				eprint!("\r    Building LUT: {}/256...", r);
			}

			for g in 0..256 {
				for b in 0..256 {
					// Convert grid cell RGB to OKLab
					let cell_rgb = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
					let cell_oklab = rgb_to_oklab(cell_rgb);

					// Find closest color in unique palette
					let mut best_index = 0;
					let mut best_distance_sq = f32::MAX;

					for (i, unique_color) in unique_colors.iter().enumerate() {
						let dl = cell_oklab[0] - unique_color.oklab[0];
						let da = cell_oklab[1] - unique_color.oklab[1];
						let db = cell_oklab[2] - unique_color.oklab[2];

						let distance_sq = if euclidean {
							dl * dl + da * da + db * db
						} else {
							(lightness_weight * dl) * (lightness_weight * dl)
								+ da * da + db * db
						};

						if distance_sq < best_distance_sq {
							best_distance_sq = distance_sq;
							best_index = i;
						}
					}

					// Store palette position in grid
					let grid_idx = r * 256 * 256 + g * 256 + b;
					grid[grid_idx] = unique_colors[best_index].palette_position as u32;
				}
			}
		}
		eprintln!("\r    Building LUT: 256/256... Done!");

		PaletteLUT {
			grid,
			unique_colors,
			source_oklab_by_position,
		}
	}

	fn lookup(&self, r: u8, g: u8, b: u8) -> usize {
		let idx = (r as usize) * 256 * 256 + (g as usize) * 256 + (b as usize);
		self.grid[idx] as usize
	}
}

struct TargetPalette {
	oklab_colors: Vec<[f32; 3]>,
}

impl TargetPalette {
	fn from_image(img: &image::DynamicImage) -> Self {
		let rgba = img.to_rgba8();
		let raw = rgba.as_raw();
		let pixel_count = (rgba.width() * rgba.height()) as usize;
		let mut oklab_colors = Vec::with_capacity(pixel_count);

		for i in 0..pixel_count {
			let idx = i * 4;
			let rgb = [
				raw[idx] as f32 / 255.0,
				raw[idx + 1] as f32 / 255.0,
				raw[idx + 2] as f32 / 255.0,
			];
			oklab_colors.push(rgb_to_oklab(rgb));
		}

		TargetPalette { oklab_colors }
	}
}

pub struct ColorMapper {}

impl ColorMapper {
	pub fn process(
		source_pal_path: &Path,
		target_pal_path: &Path,
		input_path: &Path,
		output_path: &Path,
		euclidean: bool,
		lightness_weight: f32,
	) -> Result<()> {
		// Load images
		println!("Loading source palette...");
		let source_pal = image::open(source_pal_path)?;
		println!("Loading target palette...");
		let target_pal = image::open(target_pal_path)?;
		println!("Loading input image...");
		let input_image = image::open(input_path)?;

		// Validate palette dimensions match
		let (src_w, src_h) = source_pal.dimensions();
		let (tgt_w, tgt_h) = target_pal.dimensions();
		if src_w != tgt_w || src_h != tgt_h {
			bail!(
				"Palette dimensions mismatch: source {}x{}, target {}x{}",
				src_w,
				src_h,
				tgt_w,
				tgt_h
			);
		}

		// Build source palette LUT
		println!(
			"Building source palette LUT ({}x{} = {} pixels)...",
			src_w,
			src_h,
			src_w * src_h
		);
		let source_lut = PaletteLUT::from_image(&source_pal, euclidean, lightness_weight);

		// Build target palette cache (simple array indexed by position)
		println!("Building target palette cache...");
		let target_palette = TargetPalette::from_image(&target_pal);

		// Convert input to rgba8 and get raw buffer
		let input_rgba = input_image.to_rgba8();
		let (width, height) = input_rgba.dimensions();
		let input_raw = input_rgba.as_raw();

		// Create raw output buffer
		let pixel_count = (width * height) as usize;
		let mut output_raw = vec![0u8; pixel_count * 4];

		// Process each pixel
		println!("Processing {}x{} pixels...", width, height);
		for y in 0..height {
			// Progress indicator every 100 lines
			if y % 100 == 0 {
				eprint!("\r  Line {}/{}...", y, height);
			}

			for x in 0..width {
				let pixel_idx = (y * width + x) as usize;
				let byte_idx = pixel_idx * 4;

				// Read from input buffer (u8 RGB values)
				let r = input_raw[byte_idx];
				let g = input_raw[byte_idx + 1];
				let b = input_raw[byte_idx + 2];
				let alpha = input_raw[byte_idx + 3];

				// O(1) lookup to find palette position
				let palette_pos = source_lut.lookup(r, g, b);

				// Convert input pixel to OKLab for delta calculation
				let input_rgb = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
				let input_oklab = rgb_to_oklab(input_rgb);

				// Get source palette color at matched position
				let source_match = source_lut.source_oklab_by_position[palette_pos];

				// Calculate delta in OKLab space
				let delta = [
					input_oklab[0] - source_match[0],
					input_oklab[1] - source_match[1],
					input_oklab[2] - source_match[2],
				];

				// Get corresponding color from target palette at same position
				let target_base = target_palette.oklab_colors[palette_pos];

				// Apply delta to target color with gamut scaling
				let mut output_oklab = [
					target_base[0] + delta[0],
					target_base[1] + delta[1],
					target_base[2] + delta[2],
				];

				// Try to scale delta if out of gamut
				let mut scale = 1.0;
				loop {
					let test_linear_rgb = oklab_to_linear_rgb_unclamped(output_oklab);

					// Check if in gamut (linear RGB in [0,1])
					let is_valid = test_linear_rgb[0] >= 0.0
						&& test_linear_rgb[0] <= 1.0 && test_linear_rgb[1] >= 0.0
						&& test_linear_rgb[1] <= 1.0 && test_linear_rgb[2] >= 0.0
						&& test_linear_rgb[2] <= 1.0;

					if is_valid || scale <= 0.1 {
						// Either valid or we've scaled down enough
						break;
					}

					// Scale down delta
					scale -= 0.1;
					output_oklab = [
						target_base[0] + delta[0] * scale,
						target_base[1] + delta[1] * scale,
						target_base[2] + delta[2] * scale,
					];
				}

				// Convert back to RGB
				let output_rgb = oklab_to_rgb(output_oklab);

				// Write directly to output buffer
				output_raw[byte_idx] = (output_rgb[0] * 255.0).round() as u8;
				output_raw[byte_idx + 1] = (output_rgb[1] * 255.0).round() as u8;
				output_raw[byte_idx + 2] = (output_rgb[2] * 255.0).round() as u8;
				output_raw[byte_idx + 3] = alpha;
			}
		}
		eprintln!("\r  Line {}/{}... Done!", height, height);

		// Convert raw buffer to RgbaImage
		let output = RgbaImage::from_raw(width, height, output_raw)
			.ok_or_else(|| anyhow::anyhow!("Failed to create output image from raw buffer"))?;

		// Save output
		println!("Saving output...");
		output.save(output_path)?;
		println!("Done!");

		Ok(())
	}
}
