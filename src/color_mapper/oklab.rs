// OKLab color space conversion functions
// Based on Björn Ottosson's OKLab specification
// Reference implementation from LowTexPal project

use std::sync::OnceLock;

// Lookup tables for gamma correction (computed once on first use)
static SRGB_TO_LINEAR_TABLE: OnceLock<[f32; 256]> = OnceLock::new();
static LINEAR_TO_SRGB_TABLE: OnceLock<[f32; 256]> = OnceLock::new();

fn get_srgb_to_linear_table() -> &'static [f32; 256] {
	SRGB_TO_LINEAR_TABLE.get_or_init(|| {
		let mut table = [0.0f32; 256];
		for i in 0..256 {
			let c = i as f32 / 255.0;
			table[i] = if c <= 0.04045 {
				c / 12.92
			} else {
				((c + 0.055) / 1.055).powf(2.4)
			};
		}
		table
	})
}

fn get_linear_to_srgb_table() -> &'static [f32; 256] {
	LINEAR_TO_SRGB_TABLE.get_or_init(|| {
		let mut table = [0.0f32; 256];
		for i in 0..256 {
			let c = i as f32 / 255.0;
			table[i] = if c <= 0.0031308 {
				c * 12.92
			} else {
				1.055 * c.powf(1.0 / 2.4) - 0.055
			};
		}
		table
	})
}

// M1: Linear sRGB to LMS
const M1: [[f32; 3]; 3] = [
	[0.4122214708, 0.5363325363, 0.0514459929],
	[0.2119034982, 0.6806995451, 0.1073969566],
	[0.0883024619, 0.2817188376, 0.6299787005],
];

// M1^-1: LMS to Linear sRGB
const M1_INV: [[f32; 3]; 3] = [
	[4.0767245293, -3.3077216883, 0.2309759054],
	[-1.2681437731, 2.6093323231, -0.3411344290],
	[-0.0041119885, -0.7034763098, 1.7068625689],
];

// M2: L'M'S' to OKLab
const M2: [[f32; 3]; 3] = [
	[0.2104542553, 0.7936177850, -0.0040720468],
	[1.9779984951, -2.4285922050, 0.4505937099],
	[0.0259040371, 0.7827717662, -0.8086757660],
];

// M2^-1: OKLab to L'M'S'
const M2_INV: [[f32; 3]; 3] = [
	[1.0000000000, 0.3963377774, 0.2158037573],
	[1.0000000000, -0.1055613458, -0.0638541728],
	[1.0000000000, -0.0894841775, -1.2914855480],
];

// sRGB gamma correction: sRGB to linear RGB (using lookup table)
fn srgb_to_linear(c: f32) -> f32 {
	let table = get_srgb_to_linear_table();
	let idx = (c * 255.0).round() as usize;
	table[idx.min(255)]
}

// sRGB gamma correction: linear RGB to sRGB (using lookup table)
fn linear_to_srgb(c: f32) -> f32 {
	let table = get_linear_to_srgb_table();
	let idx = (c * 255.0).round() as usize;
	table[idx.min(255)]
}

// Matrix multiplication helper: 3x3 matrix * 3x1 vector
fn matrix_mul_3x3(matrix: &[[f32; 3]; 3], vec: [f32; 3]) -> [f32; 3] {
	[
		matrix[0][0] * vec[0] + matrix[0][1] * vec[1] + matrix[0][2] * vec[2],
		matrix[1][0] * vec[0] + matrix[1][1] * vec[1] + matrix[1][2] * vec[2],
		matrix[2][0] * vec[0] + matrix[2][1] * vec[1] + matrix[2][2] * vec[2],
	]
}

/// Convert sRGB (0-1 range) to OKLab
/// Input: [r, g, b] where each component is in range 0.0-1.0
/// Output: [L, a, b] in OKLab color space
pub fn rgb_to_oklab(rgb: [f32; 3]) -> [f32; 3] {
	// 1. sRGB to linear RGB
	let r_lin = srgb_to_linear(rgb[0]);
	let g_lin = srgb_to_linear(rgb[1]);
	let b_lin = srgb_to_linear(rgb[2]);

	// 2. Linear RGB to LMS
	let lms = matrix_mul_3x3(&M1, [r_lin, g_lin, b_lin]);

	// 3. Apply cube root to each LMS component
	let l_prime = lms[0].cbrt();
	let m_prime = lms[1].cbrt();
	let s_prime = lms[2].cbrt();

	// 4. L'M'S' to OKLab
	matrix_mul_3x3(&M2, [l_prime, m_prime, s_prime])
}

/// Convert OKLab to sRGB (0-1 range)
/// Input: [L, a, b] in OKLab color space
/// Output: [r, g, b] where each component is in range 0.0-1.0 (clamped)
pub fn oklab_to_rgb(lab: [f32; 3]) -> [f32; 3] {
	// 1. OKLab to L'M'S'
	let lms_prime = matrix_mul_3x3(&M2_INV, lab);

	// 2. Cube each component
	let l = lms_prime[0].powi(3);
	let m = lms_prime[1].powi(3);
	let s = lms_prime[2].powi(3);

	// 3. LMS to linear RGB
	let rgb_lin = matrix_mul_3x3(&M1_INV, [l, m, s]);

	// 4. Linear RGB to sRGB with clamping
	[
		linear_to_srgb(rgb_lin[0].max(0.0).min(1.0)),
		linear_to_srgb(rgb_lin[1].max(0.0).min(1.0)),
		linear_to_srgb(rgb_lin[2].max(0.0).min(1.0)),
	]
}

/// Convert OKLab to linear RGB (unclamped)
/// Input: [L, a, b] in OKLab color space
/// Output: [r, g, b] in linear RGB (may be out of [0,1] range)
/// Used for gamut testing - does not apply gamma correction or clamping
pub fn oklab_to_linear_rgb_unclamped(lab: [f32; 3]) -> [f32; 3] {
	// 1. OKLab to L'M'S'
	let lms_prime = matrix_mul_3x3(&M2_INV, lab);

	// 2. Cube each component
	let l = lms_prime[0].powi(3);
	let m = lms_prime[1].powi(3);
	let s = lms_prime[2].powi(3);

	// 3. LMS to linear RGB (NO CLAMPING, NO GAMMA)
	matrix_mul_3x3(&M1_INV, [l, m, s])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_black_roundtrip() {
		let black = [0.0, 0.0, 0.0];
		let lab = rgb_to_oklab(black);
		let rgb = oklab_to_rgb(lab);
		assert!((rgb[0] - black[0]).abs() < 0.0001);
		assert!((rgb[1] - black[1]).abs() < 0.0001);
		assert!((rgb[2] - black[2]).abs() < 0.0001);
	}

	#[test]
	fn test_white_roundtrip() {
		let white = [1.0, 1.0, 1.0];
		let lab = rgb_to_oklab(white);
		let rgb = oklab_to_rgb(lab);
		assert!((rgb[0] - white[0]).abs() < 0.0001);
		assert!((rgb[1] - white[1]).abs() < 0.0001);
		assert!((rgb[2] - white[2]).abs() < 0.0001);
	}

	#[test]
	fn test_red_roundtrip() {
		let red = [1.0, 0.0, 0.0];
		let lab = rgb_to_oklab(red);
		let rgb = oklab_to_rgb(lab);
		assert!((rgb[0] - red[0]).abs() < 0.0001);
		assert!((rgb[1] - red[1]).abs() < 0.0001);
		assert!((rgb[2] - red[2]).abs() < 0.0001);
	}
}
