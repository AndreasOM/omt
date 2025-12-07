# Benchmarks

## omt-color-mapper

```sh
cargo run --release --bin omt-color-mapper -- benchmark --colors 100000 --image-size 4096
```

### 2025-12-07

Running benchmark:
  colors          : 100000
  image_size      : 4096x4096
  euclidean       : false
  lightness_weight: 2

Generating random source palette (100000 colors)...
Generating random target palette (100000 colors)...
Generating random test image (4096x4096)...

=== Starting benchmark ===

Building source palette LUT (100000 colors)...
  Deduplicating palette colors...
  Found 99706 unique colors (reduced from 100000 pixels)
  Building 256^3 lookup table (16.7M entries)...
    Building LUT: 256/256... Done!
LUT build time: 2599.948s

Building target palette cache...
Target cache time: 0.006s

Processing 4096x4096 pixels...
  Line 4096/4096... Done!
Processing time: 3.114s

=== Benchmark Results ===
LUT build time    : 2599.948s
Target cache time : 0.006s
Processing time   : 3.114s
Total time        : 2603.740s
Image size        : 4096x4096 (16.78 MP)
Throughput        : 5.39 MP/s
Unique colors     : 99706

Running benchmark:
  colors          : 1000
  image_size      : 4096x4096
  euclidean       : false
  lightness_weight: 2

Generating random source palette (1000 colors)...
Generating random target palette (1000 colors)...
Generating random test image (4096x4096)...

=== Starting benchmark ===

Building source palette LUT (1000 colors)...
  Deduplicating palette colors...
  Found 1000 unique colors (reduced from 1000 pixels)
  Building 256^3 lookup table (16.7M entries)...
    Building LUT: 256/256... Done!
LUT build time: 26.141s

Building target palette cache...
Target cache time: 0.000s

Processing 4096x4096 pixels...
  Line 4096/4096... Done!
Processing time: 2.631s

=== Benchmark Results ===
LUT build time    : 26.141s
Target cache time : 0.000s
Processing time   : 2.631s
Total time        : 29.170s
Image size        : 4096x4096 (16.78 MP)
Throughput        : 6.38 MP/s
Unique colors     : 1000

```
cargo run --release --bin omt-color-mapper -- benchmark --colors 1000 --image-size 4096 --oneline
```
[2025-12-25 17:26:58] [1000] [4096x4096] - 21.225s 19.588s 1.365s 12.29MP/s

Note: From here on just the outputs of the benchmarks.

[2025-12-25 17:33:10] [9996] [4096x4096] - 194.593s 192.813s 1.316s 12.75MP/s

Create initial lookup cube via kd-tree.
[2025-12-25 17:53:39] [9997] [4096x4096] - 5.990s 4.428s 1.284s 13.07MP/s
[2025-12-25 17:54:16] [99716] [4096x4096] - 6.894s 5.193s 1.401s 11.98MP/s

Parallel LUT creation
[2025-12-25 18:21:55] [970813] [4096x4096] - 3.354s 1.003s 2.055s 8.16MP/s
[2025-12-25 18:24:47] [970781] [16384x16384] - 41.735s 1.304s 35.733s 7.51MP/s

Parallal mapping
[2025-12-25 18:35:35] [970485] [8192x8192] - 3.417s 1.063s 1.341s 50.03MP/s
[2025-12-25 18:36:20] [970489] [16384x16384] - 11.666s 1.129s 6.153s 43.63MP/s