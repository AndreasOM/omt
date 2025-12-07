# OMT Color Mapper Examples

The `omt-color-mapper` tool maps colors from one palette to another while preserving perceptual color distances using the OKLab color space.

## Example: Fish Character Variants

This example demonstrates how to create different color variants of a character sprite by mapping from a source palette to various target palettes.

### Available Palettes

| Bronze (Source) | Silver | Gold | Diamond | Ruby |
|-----------------|--------|------|---------|------|
| ![Bronze Palette](pal-bronze.png) | ![Silver Palette](pal-silver.png) | ![Gold Palette](pal-gold.png) | ![Diamond Palette](pal-diamond.png) | ![Ruby Palette](pal-ruby.png) |

## Color Mapping Examples

### Silver Variant

Command:
```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-silver.png --input fiiish-bronze.png --output fiiish-silver.png
```

| Source | Palette | Result |
|--------|---------|--------|
| ![Bronze Fish](fiiish-bronze.png) | ![Silver Palette](pal-silver.png) | ![Silver Fish](fiiish-silver.png) |

### Gold Variant

Command:
```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-gold.png --input fiiish-bronze.png --output fiiish-gold.png
```

| Source | Palette | Result |
|--------|---------|--------|
| ![Bronze Fish](fiiish-bronze.png) | ![Gold Palette](pal-gold.png) | ![Gold Fish](fiiish-gold.png) |

### Diamond Variant

Command:
```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-diamond.png --input fiiish-bronze.png --output fiiish-diamond.png
```

| Source | Palette | Result |
|--------|---------|--------|
| ![Bronze Fish](fiiish-bronze.png) | ![Diamond Palette](pal-diamond.png) | ![Diamond Fish](fiiish-diamond.png) |

### Ruby Variant

Command:
```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-ruby.png --input fiiish-bronze.png --output fiiish-ruby.png
```

| Source | Palette | Result |
|--------|---------|--------|
| ![Bronze Fish](fiiish-bronze.png) | ![Ruby Palette](pal-ruby.png) | ![Ruby Fish](fiiish-ruby.png) |

## Batch Generation

See `generate-fish-variants.sh` for a script that generates all variants at once.
