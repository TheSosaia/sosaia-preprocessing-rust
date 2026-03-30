# Sosaia Preprocessing Pipeline

## Quick Start
```bash
source "$HOME/.cargo/env"
cargo build --release
./target/release/sosaia-preprocessing \
  --bbox "48.856,2.351,48.858,2.354" \
  --city-name test-paris \
  --output-dir ./output \
  --file ./testdata/sample_overpass.json
```

## Architecture
- Fetches OSM data from Overpass API (or local JSON file)
- Parses into Minecraft-style blocks via arnis core logic
- Exports two formats per chunk:
  - `chunks/` — raw block data (.bin, 6 bytes/block) for physics collision
  - `meshes/` — precomputed greedy-meshed vertex buffers (.mesh.bin) with LOD for zero-runtime-meshing

## Key Files
- `src/main.rs` — pipeline orchestration (7 steps)
- `src/chunk_export.rs` — block export + mesh export + LOD integration
- `src/greedy_mesher.rs` — Rust greedy mesher (Lysenko algorithm) + LOD downsampling
- `src/mesh_export.rs` — binary mesh format writer (SOSM format)
- `src/material_ids.rs` — Minecraft block → Sosaia material ID mapping (0-131)
- `src/data_processing.rs` — OSM element processing, world generation

## Output Structure
```
output/
  chunks/<city>/
    manifest.json
    0_0.bin              # raw blocks for physics
  meshes/<city>/
    manifest.json
    0_0.mesh.bin         # LOD0 — full detail
    0_0.lod1.mesh.bin    # LOD1 — 2x2x2 downsample (~7x fewer verts)
    0_0.lod2.mesh.bin    # LOD2 — 4x4x4 downsample (~40x fewer verts)
```

## Mesh Binary Format (.mesh.bin)
```
Header (10 bytes):
  magic:       u32 LE = 0x534F534D ("SOSM")
  version:     u16 LE = 1
  chunk_x:     i16 LE
  chunk_z:     i16 LE
Material count: u16 LE
Per material:
  material_id:    u16 LE
  vertex_count:   u32 LE
  index_count:    u32 LE
  positions:      [f32 LE; vertex_count * 3]
  normals:        [f32 LE; vertex_count * 3]
  colors:         [f32 LE; vertex_count * 3]  (AO baked)
  indices:        [u32 LE; index_count]
```

## LOD System
Three LOD levels generated per chunk during preprocessing:
- **LOD0** (no suffix): Full detail greedy mesh
- **LOD1** (.lod1): 2x2x2 block merge → ~7x fewer vertices
- **LOD2** (.lod2): 4x4x4 block merge → ~40x fewer vertices

The web client uses THREE.LOD to auto-switch based on camera distance:
- LOD0 at <400m, LOD1 at 400-1000m, LOD2 at >1000m

## Large Area Processing
Overpass API has size limits. For large areas (e.g. Manhattan), split into ~2km x 2km tiles:
```bash
# Fetch each tile separately with --save-json-file for offline reuse
./target/release/sosaia-preprocessing \
  --bbox "lat1,lng1,lat2,lng2" \
  --city-name manhattan-tile0 \
  --output-dir ./output \
  --terrain \
  --save-json-file ./testdata/manhattan-tile0.json
```

## Related Repos
- `sosaia-web-ts` — Next.js + Three.js voxel city viewer (consumer of chunk/mesh data)
- `sosaia-backend-go` — Go WebSocket server for multiplayer
