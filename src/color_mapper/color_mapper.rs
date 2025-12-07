use std::collections::HashSet;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use image::{GenericImageView, RgbaImage};
use kiddo::ImmutableKdTree;
use kiddo::SquaredEuclidean;
use rand::Rng;

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

		// Build KD-tree for fast nearest-neighbor lookup
		println!("  Building KD-tree from {} unique colors...", unique_colors.len());
		let mut tree_points = Vec::with_capacity(unique_colors.len());
		for unique_color in &unique_colors {
			let scaled_point = if euclidean {
				// Euclidean: use original OKLab coordinates
				unique_color.oklab
			} else {
				// Weighted: pre-scale L coordinate to make euclidean distance equivalent to weighted
				[
					unique_color.oklab[0] * lightness_weight,
					unique_color.oklab[1],
					unique_color.oklab[2],
				]
			};
			tree_points.push(scaled_point);
		}
		let kdtree: ImmutableKdTree<f32, 3> = ImmutableKdTree::new_from_slice(&tree_points);

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

					// Apply same scaling as tree
					let query_point = if euclidean {
						cell_oklab
					} else {
						[
							cell_oklab[0] * lightness_weight,
							cell_oklab[1],
							cell_oklab[2],
						]
					};

					// Find closest color using KD-tree (O(log N) instead of O(N))
					let nearest = kdtree.nearest_one::<SquaredEuclidean>(&query_point);
					let best_index = nearest.item as usize;

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

	pub fn benchmark(
		colors: usize,
		image_size: u32,
		euclidean: bool,
		lightness_weight: f32,
		oneline: bool,
	) -> Result<()> {
		let mut rng = rand::thread_rng();
		let start_total = Instant::now();

		// Generate random source palette (single row of colors × 1 pixel)
		println!("Generating random source palette ({} colors)...", colors);
		let mut source_pal_raw = vec![0u8; colors * 4];
		for i in 0..colors {
			let idx = i * 4;
			source_pal_raw[idx] = rng.gen::<u8>();
			source_pal_raw[idx + 1] = rng.gen::<u8>();
			source_pal_raw[idx + 2] = rng.gen::<u8>();
			source_pal_raw[idx + 3] = 255;
		}
		let source_pal = image::DynamicImage::ImageRgba8(
			RgbaImage::from_raw(colors as u32, 1, source_pal_raw)
				.ok_or_else(|| anyhow::anyhow!("Failed to create source palette"))?,
		);

		// Generate random target palette (single row of colors × 1 pixel)
		println!("Generating random target palette ({} colors)...", colors);
		let mut target_pal_raw = vec![0u8; colors * 4];
		for i in 0..colors {
			let idx = i * 4;
			target_pal_raw[idx] = rng.gen::<u8>();
			target_pal_raw[idx + 1] = rng.gen::<u8>();
			target_pal_raw[idx + 2] = rng.gen::<u8>();
			target_pal_raw[idx + 3] = 255;
		}
		let target_pal = image::DynamicImage::ImageRgba8(
			RgbaImage::from_raw(colors as u32, 1, target_pal_raw)
				.ok_or_else(|| anyhow::anyhow!("Failed to create target palette"))?,
		);

		// Generate random test input image
		println!(
			"Generating random test image ({}x{})...",
			image_size, image_size
		);
		let pixel_count = (image_size * image_size) as usize;
		let mut input_raw = vec![0u8; pixel_count * 4];
		for i in 0..pixel_count {
			let idx = i * 4;
			input_raw[idx] = rng.gen::<u8>();
			input_raw[idx + 1] = rng.gen::<u8>();
			input_raw[idx + 2] = rng.gen::<u8>();
			input_raw[idx + 3] = 255;
		}
		let input_image = image::DynamicImage::ImageRgba8(
			RgbaImage::from_raw(image_size, image_size, input_raw)
				.ok_or_else(|| anyhow::anyhow!("Failed to create input image"))?,
		);

		println!();
		println!("=== Starting benchmark ===");
		println!();

		// Build source palette LUT
		let start_lut = Instant::now();
		println!("Building source palette LUT ({} colors)...", colors);
		let source_lut = PaletteLUT::from_image(&source_pal, euclidean, lightness_weight);
		let lut_time = start_lut.elapsed();
		println!("LUT build time: {:.3}s", lut_time.as_secs_f64());
		println!();

		// Build target palette cache
		let start_target = Instant::now();
		println!("Building target palette cache...");
		let target_palette = TargetPalette::from_image(&target_pal);
		let target_time = start_target.elapsed();
		println!("Target cache time: {:.3}s", target_time.as_secs_f64());
		println!();

		// Convert input to rgba8 and get raw buffer
		let input_rgba = input_image.to_rgba8();
		let (width, height) = input_rgba.dimensions();
		let input_raw_buf = input_rgba.as_raw();

		// Create raw output buffer
		let pixel_count = (width * height) as usize;
		let mut output_raw = vec![0u8; pixel_count * 4];

		// Process each pixel
		let start_process = Instant::now();
		println!("Processing {}x{} pixels...", width, height);
		for y in 0..height {
			if y % 100 == 0 {
				eprint!("\r  Line {}/{}...", y, height);
			}

			for x in 0..width {
				let pixel_idx = (y * width + x) as usize;
				let byte_idx = pixel_idx * 4;

				let r = input_raw_buf[byte_idx];
				let g = input_raw_buf[byte_idx + 1];
				let b = input_raw_buf[byte_idx + 2];
				let alpha = input_raw_buf[byte_idx + 3];

				let palette_pos = source_lut.lookup(r, g, b);

				let input_rgb = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
				let input_oklab = rgb_to_oklab(input_rgb);

				let source_match = source_lut.source_oklab_by_position[palette_pos];

				let delta = [
					input_oklab[0] - source_match[0],
					input_oklab[1] - source_match[1],
					input_oklab[2] - source_match[2],
				];

				let target_base = target_palette.oklab_colors[palette_pos];

				let mut output_oklab = [
					target_base[0] + delta[0],
					target_base[1] + delta[1],
					target_base[2] + delta[2],
				];

				let mut scale = 1.0;
				loop {
					let test_linear_rgb = oklab_to_linear_rgb_unclamped(output_oklab);

					let is_valid = test_linear_rgb[0] >= 0.0
						&& test_linear_rgb[0] <= 1.0 && test_linear_rgb[1] >= 0.0
						&& test_linear_rgb[1] <= 1.0 && test_linear_rgb[2] >= 0.0
						&& test_linear_rgb[2] <= 1.0;

					if is_valid || scale <= 0.1 {
						break;
					}

					scale -= 0.1;
					output_oklab = [
						target_base[0] + delta[0] * scale,
						target_base[1] + delta[1] * scale,
						target_base[2] + delta[2] * scale,
					];
				}

				let output_rgb = oklab_to_rgb(output_oklab);

				output_raw[byte_idx] = (output_rgb[0] * 255.0).round() as u8;
				output_raw[byte_idx + 1] = (output_rgb[1] * 255.0).round() as u8;
				output_raw[byte_idx + 2] = (output_rgb[2] * 255.0).round() as u8;
				output_raw[byte_idx + 3] = alpha;
			}
		}
		let process_time = start_process.elapsed();
		eprintln!("\r  Line {}/{}... Done!", height, height);
		println!("Processing time: {:.3}s", process_time.as_secs_f64());
		println!();

		let total_time = start_total.elapsed();
		let megapixels = (width * height) as f64 / 1_000_000.0;
		let throughput = megapixels / process_time.as_secs_f64();

		if oneline {
			// Format: [YYYY-MM-DD HH:MM:SS] [colors] [widthxheight] - [total_time]s [lut_time]s [processing_time]s [throughput]MP/s
			let now = SystemTime::now();
			let duration = now.duration_since(UNIX_EPOCH).unwrap();
			let secs = duration.as_secs();

			// Convert to date/time (simplified UTC calculation)
			let days_since_epoch = secs / 86400;
			let remaining_secs = secs % 86400;
			let hours = remaining_secs / 3600;
			let minutes = (remaining_secs % 3600) / 60;
			let seconds = remaining_secs % 60;

			// Approximate date calculation (good enough for logging)
			let year = 1970 + (days_since_epoch / 365);
			let day_of_year = days_since_epoch % 365;
			let month = (day_of_year / 30) + 1;
			let day = (day_of_year % 30) + 1;

			println!(
				"[{:04}-{:02}-{:02} {:02}:{:02}:{:02}] [{}] [{}x{}] - {:.3}s {:.3}s {:.3}s {:.2}MP/s",
				year, month, day, hours, minutes, seconds,
				source_lut.unique_colors.len(),
				width, height,
				total_time.as_secs_f64(),
				lut_time.as_secs_f64(),
				process_time.as_secs_f64(),
				throughput
			);
		} else {
			println!("=== Benchmark Results ===");
			println!("LUT build time    : {:.3}s", lut_time.as_secs_f64());
			println!("Target cache time : {:.3}s", target_time.as_secs_f64());
			println!("Processing time   : {:.3}s", process_time.as_secs_f64());
			println!("Total time        : {:.3}s", total_time.as_secs_f64());
			println!("Image size        : {}x{} ({:.2} MP)", width, height, megapixels);
			println!("Throughput        : {:.2} MP/s", throughput);
			println!("Unique colors     : {}", source_lut.unique_colors.len());
		}

		Ok(())
	}
}
