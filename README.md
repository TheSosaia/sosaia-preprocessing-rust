# Sosaia Preprocessing Pipeline

Converts real-world geographic data (OpenStreetMap) into binary chunk files for the Sosaia Unity client.

Based on [arnis](https://github.com/louis-e/arnis) core logic, with Minecraft output replaced by Sosaia's binary chunk format.

## Prerequisites

- **Rust toolchain** (stable)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Build

```bash
cd sosaia-preprocessing

# Debug build (faster compile, slower runtime)
cargo build

# Release build (slower compile, faster runtime — use this for generating cities)
cargo build --release
```

## Run

### With network (fetches data from Overpass API)

```bash
./target/release/sosaia-pipeline \
  --bbox "48.856,2.351,48.858,2.354" \
  --city-name paris \
  --output-dir ./output \
  --terrain
```

### Without network (using a local OSM JSON file)

```bash
./target/release/sosaia-pipeline \
  --bbox "48.856,2.351,48.858,2.354" \
  --city-name paris \
  --output-dir ./output \
  --file ./testdata/sample_overpass.json
```

A sample test file is included at `testdata/sample_overpass.json`.

### Full options

```
OPTIONS:
  --bbox <BBOX>              Bounding box: min_lat,min_lng,max_lat,max_lng (required)
  --city-name <NAME>         City name for output folder (optional, defaults to "city")
  --output-dir <PATH>        Output directory (optional, defaults to ./output)
  --file <PATH>              Local OSM JSON file instead of fetching from Overpass API
  --terrain                  Enable elevation/terrain generation
  --scale <FLOAT>            Blocks per meter (default: 1.0)
  --interior [true|false]    Generate building interiors (default: true)
  --roof [true|false]        Generate building roofs (default: true)
  --city-boundaries [true|false]  Urban ground detection (default: true)
  --debug                    Verbose output
  --timeout <SECONDS>        Floodfill timeout
  --save-json-file <PATH>    Save downloaded OSM data to file for offline reuse
```

## Output

```
output/
  chunks/
    <city-name>/
      manifest.json       # City metadata + chunk index
      0_0.bin             # Binary chunk file
      0_1.bin
      1_0.bin
      ...
```

### manifest.json

```json
{
  "city": "paris",
  "bbox": { "min_lat": 48.856, "min_lng": 2.351, "max_lat": 48.858, "max_lng": 2.354 },
  "chunk_size_meters": 200,
  "chunks": [
    { "x": 0, "z": 0, "file": "0_0.bin", "block_count": 160519, "size_bytes": 963132 }
  ],
  "total_blocks": 187699
}
```

### Binary chunk format (.bin)

```
Header (14 bytes):
  magic:       u32 LE = 0x534F5341 ("SOSA")
  version:     u16 LE = 1
  chunk_x:     i32 LE
  chunk_z:     i32 LE

Block count:   u32 LE

Block data (6 bytes per block, repeated block_count times):
  local_x:     u8       (0-199, position within chunk)
  local_z:     u8       (0-199, position within chunk)
  y:           i16 LE   (height)
  material_id: u16 LE   (Sosaia material ID)
```

### Material ID table

See `src/material_ids.rs` for the full mapping. Summary:

| Range   | Category       | Examples                    |
|---------|----------------|-----------------------------|
| 0       | Air            | Skip rendering              |
| 1-9     | Terrain        | grass, dirt, sand, water     |
| 10-19   | Roads          | asphalt, cobblestone         |
| 20-49   | Building walls | brick, concrete, sandstone   |
| 50-69   | Roofs          | tile, slate, flat concrete   |
| 70-79   | Glass/windows  | clear, tinted, colored       |
| 80-89   | Details        | doors, fences, rails         |
| 90-99   | Nature         | tree trunks, leaves          |
| 100-109 | Interior       | bookshelf, bed, chest        |
| 110+    | Misc           | bedrock, glowstone, flowers  |

## Example: save OSM data for offline use

```bash
# Step 1: Download and save (requires network)
./target/release/sosaia-pipeline \
  --bbox "35.68,139.76,35.69,139.77" \
  --city-name tokyo \
  --output-dir ./output \
  --save-json-file ./testdata/tokyo.json \
  --terrain

# Step 2: Re-run offline anytime
./target/release/sosaia-pipeline \
  --bbox "35.68,139.76,35.69,139.77" \
  --city-name tokyo \
  --output-dir ./output \
  --file ./testdata/tokyo.json
```

## Quick test (no network required)

```bash
cargo build --release

./target/release/sosaia-pipeline \
  --bbox "48.856,2.351,48.858,2.354" \
  --city-name test-paris \
  --output-dir ./output \
  --file ./testdata/sample_overpass.json

# Verify output
ls -lh ./output/chunks/test-paris/
cat ./output/chunks/test-paris/manifest.json
```

Expected output: 4 chunks, ~187K blocks, ~1MB total.
