# Status
![Rust](https://github.com/AndreasOM/omt/workflows/Rust/badge.svg)

# omt
Omni-Mad Tools

A set of tiny tools used for game development.

## omt-asset

Controls the conversion of source _content_ into game _data_.

## omt-atlas

Combines textures into one (or more) texture atlases.

## omt-font

Creates a signed distance field (SDF) font from a font file, e.g. .ttf, o .otf.

## omt-packer

Combines a list of files into a single pakfile.

## omt-script

Verifies, and copies (aka _converts_) a script.
Currently very rough, only does a quick lua verification.

## omt-shader
Verififes, and optionally optimizes shaders.

## omt-soundbank

Converts a soundbank configurtion into a binary config file, and writes out a header (currently c++ only) so you can use #defined values instead of magic numbers.

## omt-xcassets

Creates an AppIcon set from a single file. Mostly used to make development for Apple devices easier.

