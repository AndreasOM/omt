#[cfg(test)]
mod tests {
	//    use super::*;
	use crate::gfx::DrawBuffer;

	#[test]
	fn mix_rgba_works_for_black_on_black() {
		let fg = 0x00000000;
		let bg = 0x00000000;
		let result = DrawBuffer::mix_rgba(fg, bg, 0.0);
		assert!(result == 0x00000000);
		let result = DrawBuffer::mix_rgba(fg, bg, 1.0);
		assert!(result == 0x00000000);
	}
	#[test]
	fn mix_rgba_works_for_white_on_white() {
		let fg = 0xffffff00;
		let bg = 0xffffff00;
		let result = DrawBuffer::mix_rgba(fg, bg, 0.0);
		assert!(result == 0xffffff00);
		let result = DrawBuffer::mix_rgba(fg, bg, 1.0);
		assert!(result == 0xffffff00);
	}
	#[test]
	fn mix_rgba_works_for_white_on_black() {
		let fg = 0xffffff00;
		let bg = 0x00000000;
		let result = DrawBuffer::mix_rgba(fg, bg, 0.0);
		assert!(result == 0x00000000);
		let result = DrawBuffer::mix_rgba(fg, bg, 0.5);
		assert!(result == 0x7f7f7f00);
		let result = DrawBuffer::mix_rgba(fg, bg, 1.0);
		assert!(result == 0xffffff00);
	}
	#[test]
	fn mix_rgba_works_for_red_on_black() {
		let fg = 0xff000000;
		let bg = 0x00000000;
		let result = DrawBuffer::mix_rgba(fg, bg, 0.0);
		assert!(result == 0x00000000);
		let result = DrawBuffer::mix_rgba(fg, bg, 0.5);
		assert!(result == 0x7f000000);
		let result = DrawBuffer::mix_rgba(fg, bg, 1.0);
		assert!(result == 0xff000000);
	}
	#[test]
	fn mix_rgba_works_for_red_on_white() {
		let fg = 0xff000000;
		let bg = 0xffffff00;
		let result = DrawBuffer::mix_rgba(fg, bg, 0.0);
		assert!(result == 0xffffff00);
		let result = DrawBuffer::mix_rgba(fg, bg, 0.5);
		assert!(result == 0xff7f7f00);
		let result = DrawBuffer::mix_rgba(fg, bg, 1.0);
		assert!(result == 0xff000000);
	}
}
