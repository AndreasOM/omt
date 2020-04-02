# In Progress

[ ] Initial version of omt-shader

[ ] Create new .omfont format
	[x] Fix y-offset
	[x] Fix fixed font texture size handling for previewer
	[x] Add loader for new format to omlib
	[x] Test new font in Fiiish!
	[ ] Fix scaling issue

# TODO

[ ] Allow defining of used characters in font
[ ] Switch font previewer to use matrix for bounding boxes
[ ] Fix font size setting handling?

[ ] Cleanup font code
[ ] Cleanup atlas, and remove obsolete fitting leftovers

[ ] Reduce debug noise
[ ] Find solution for fully qualified filename, vs original filename
[ ] Only build data when content is newer

[ ] Initial version of omt-script
[ ] Initial version of omt-shape
[ ] Initial version of omt-zone 	(! Fiiish! specific)
[ ] Initial version of omt-config	(! Maybe: Fiiish! specific)


# DONE

## 2020

### April
[ ] Allow runnig exactly one specified asset_config (by passing file to content-directory)

### February

[x] Extract atlas fitting form atlas builder
[x] Initial version of omt-font
	[x] Write stub for new command
	[x] Generate images for glyphs
	[x] Put glyphs into "atlas", and then(!) rasterize into image
	[x] Write data output
	[x] Convert glyph pixels to signed distance
	[x] Try: Convert individual glyphs to signed distance before blitting them into shared texture

[x] Set distance back to 0-255 with 127 for point on edge

### January


[x] Initial version of omt-packer
[x] Initial version of omt-asset
[x] Initial version of omt-atlas
[x] Initial version of omt-soundbank

