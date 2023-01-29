# In Progress


# Suspended

[ ] Create new .omfont format
	[x] Fix y-offset
	[x] Fix fixed font texture size handling for previewer
	[x] Add loader for new format to omlib
	[x] Test new font in Fiiish!
	[ ] Fix scaling issue

# TODO

- [ ] omt-atlas: Add multi atlas support to viewer

[ ] Use crchelper functions in packer
[ ] Update packages for
	[ ] clap
	[ ] shader-crusher
	[ ] distance-field (as soon as it is released with new image (>0.24.x) dependency)
[ ] omt-xcassets: Fix filter
[ ] omt-xcassets: Implement mode fill
[ ] omt-xcassets: Implement mode update
[ ] omt-xcassets: Finalize command line interface



[ ] Initial version of omt-shape

[ ] Allow defining of used characters in font
[ ] Switch font previewer to use matrix for bounding boxes
[ ] Fix font size setting handling?

[ ] Cleanup font code
[ ] Cleanup atlas, and remove obsolete fitting leftovers

[ ] Reduce debug noise
[ ] Find solution for fully qualified filename, vs original filename
[ ] Only build data when content is newer

[ ] Initial version of omt-zone 	(! Fiiish! specific)
[ ] Initial version of omt-config	(! Maybe: Fiiish! specific)

[ ] Use return codes from tools when called from asset tool
[ ] Untangle the AssetBuilder vs ToolRun mess
[ ] Improve verification, and error reporting of asset_config.
[ ] Improve README

[ ] Print help when called without any parameters

[ ] Asset tool needs to update paklist

# DONE

## 2023

### January

#### v0.8.x

- [x] omt-atlas: Add AtlasSet to handle multiple Atlases
- [x] omt-atlas: Automagically detect size for atlas when combining
- [x] omt-atlas: Actually add border as requested


## 2022

### May

[x] Publish new version via github action
[x] Release binary packages via github actions

## 2020

### April

[x] Allow runnig exactly one specified asset_config (by passing file to content-directory)
[x] Added very basic build action to github
[x] Initial version of omt-shader
[x] Added input:basename placeholder
[x] Added globbing to input filename resolution in asset tool
[x] Allow to combine inputs for tool run
[x] Abort with error on empty, or broken asset_config
[x] Added dry-run option to asset tool
[x] Initial version of omt-script

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

