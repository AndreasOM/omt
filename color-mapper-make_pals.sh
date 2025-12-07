#!/bin/sh
# Generate upscaled target palettes:
#   color-mapper-target_pal-<bronze|silver|gold|diamond|ruby>.png

set -eu

LOWTEXPAL="${HOME}/bin/lowtexpal"   # use $HOME, not "~"
OUT_DIR="."
STEPS="${STEPS:-8}"
COLORS=$((2 + STEPS))               # black + white + steps
UPSCALE_PCT="${UPSCALE_PCT:-3200%}"
FINAL_PREFIX="color-mapper-target_pal"
OPTIPNG_BIN="$(command -v optipng || true)"

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "Error: required command '$1' not found in PATH." >&2
    exit 1
  }
}

require_cmd gm
[ -x "$LOWTEXPAL" ] || { echo "Error: $LOWTEXPAL not executable"; exit 1; }

# track temps for cleanup
TMP_LIST=""
cleanup() {
  # shellcheck disable=SC2086
  [ -n "$TMP_LIST" ] && rm -f $TMP_LIST >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

crush_png() {
  [ -n "$OPTIPNG_BIN" ] && "$OPTIPNG_BIN" -o9 -quiet -- "$1"
}

# make_variant <name> <start-hex> <end-hex>
make_variant() {
  name="$1"; start="$2"; end="$3"

  small_tmp="$(mktemp "${OUT_DIR}/.pal_${name}.XXXXXX.png")"
  big_tmp="$(mktemp   "${OUT_DIR}/.pal_big_${name}.XXXXXX.png")"
  TMP_LIST="$TMP_LIST $small_tmp $big_tmp"

  final="${OUT_DIR}/${FINAL_PREFIX}-${name}.png"

  # fresh final
  rm -f -- "$final"

  # build small palette (single-line strip)
  "$LOWTEXPAL" -f "$small_tmp" --min-width "$COLORS" add-color    --color "#000000"
  "$LOWTEXPAL" -f "$small_tmp" --min-width "$COLORS" add-color    --color "#ffffff"
  "$LOWTEXPAL" -f "$small_tmp" --min-width "$COLORS" add-gradient --start-color "$start" --end-color "$end" --steps "$STEPS"

  # upscale to final (big is temporary)
  gm convert "$small_tmp" -filter point -resize "$UPSCALE_PCT" "$big_tmp"
  mv -f -- "$big_tmp" "$final"
  crush_png "$final"

  echo "wrote: $final"
}

# --- Variants ---
make_variant bronze  "#523522" "#f4a039"
make_variant silver  "#3d3c3c" "#b4b4b4"
make_variant gold    "#8d5f3b" "#f7eb68"
make_variant diamond "#2d7692" "#3bfcfc"
make_variant ruby    "#681077" "#d51617"

# copy bronze palette as source version (already crushed)
cp color-mapper-target_pal-bronze.png color-mapper-source_pal-bronze.png
