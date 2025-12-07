#!/bin/sh
# Generate all fish color variants from bronze source

omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-silver.png --input fiiish-bronze.png --output fiiish-silver.png
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-gold.png --input fiiish-bronze.png --output fiiish-gold.png
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-diamond.png --input fiiish-bronze.png --output fiiish-diamond.png
omt-color-mapper map --source-pal pal-bronze.png --target-pal pal-ruby.png --input fiiish-bronze.png --output fiiish-ruby.png
