#!/bin/bash

# Build the binary once
echo "Building omt-color-mapper..."
cargo build --release --bin omt-color-mapper

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo ""
echo "Running color mapper for all target palettes..."
echo ""

# Source palette and input image
SOURCE_PAL="color-mapper-source_pal-bronze.png"
INPUT="color-mapper-input.png"

# Array of target palettes
TARGET_PALS=(
    "bronze"
    "silver"
    "gold"
    "diamond"
    "ruby"
)

# Run for each target palette
for target in "${TARGET_PALS[@]}"; do
    TARGET_PAL="color-mapper-target_pal-${target}.png"
    OUTPUT="color-mapper-output-${target}.png"

    echo "Processing: bronze -> ${target}"
    echo "  Source: ${SOURCE_PAL}"
    echo "  Target: ${TARGET_PAL}"
    echo "  Output: ${OUTPUT}"

    ./target/release/omt-color-mapper map \
        --source-pal "${SOURCE_PAL}" \
        --target-pal "${TARGET_PAL}" \
        --input "${INPUT}" \
        --output "${OUTPUT}"

    if [ $? -eq 0 ]; then
        echo "  ✓ Success"
    else
        echo "  ✗ Failed"
    fi
    echo ""
done

echo "Done! Generated outputs:"
ls -lh color-mapper-output-*.png 2>/dev/null || echo "No output files found"
