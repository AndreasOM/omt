# OMT Color Mapper Examples

The `omt-color-mapper` tool maps colors from one palette to another while preserving perceptual color distances using the OKLab color space.

## Example: Fish Character Variants

This example demonstrates how to create different color variants of a character sprite by mapping from a source palette to various target palettes.

### Source Image

![Bronze Fish](fiiish-bronze.png)

**fiiish-bronze.png** - Original character using the bronze palette

### Palettes

![Bronze Palette](pal-bronze.png) **pal-bronze.png** - Source palette (bronze)

![Silver Palette](pal-silver.png) **pal-silver.png** - Target palette (silver)

![Gold Palette](pal-gold.png) **pal-gold.png** - Target palette (gold)

![Diamond Palette](pal-diamond.png) **pal-diamond.png** - Target palette (diamond)

![Ruby Palette](pal-ruby.png) **pal-ruby.png** - Target palette (ruby)

## Commands

### Generate Silver Variant

```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-silver.png --input fiiish-bronze.png --output fiiish-silver.png
```

![Silver Fish](fiiish-silver.png)

### Generate Gold Variant

```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-gold.png --input fiiish-bronze.png --output fiiish-gold.png
```

![Gold Fish](fiiish-gold.png)

### Generate Diamond Variant

```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-diamond.png --input fiiish-bronze.png --output fiiish-diamond.png
```

![Diamond Fish](fiiish-diamond.png)

### Generate Ruby Variant

```bash
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-ruby.png --input fiiish-bronze.png --output fiiish-ruby.png
```

![Ruby Fish](fiiish-ruby.png)

## Batch Generation

See `generate-fish-variants.sh` for a script that generates all variants at once.
