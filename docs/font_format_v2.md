## font format V2


OMFONT == 4f 4d 46 4f 4e 54
u32 # version 2
u16 # font size
u16 # number of codepoints
x*u32 # codepoint

x*{
	mat3x2	# texture matrix
	f32		# advance
	f32		# yOffset
}
