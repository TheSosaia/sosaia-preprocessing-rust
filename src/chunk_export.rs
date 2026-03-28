/// Sosaia chunk export module.
///
/// Reads the WorldEditor's in-memory block data and exports it as
/// binary chunk files suitable for the Unity client.
///
/// Chunk file format (.bin):
///   Header (14 bytes):
///     magic:       u32 = 0x534F5341 ("SOSA")
///     version:     u16 = 1
///     chunk_x:     i32
///     chunk_z:     i32
///   Block count:   u32
///   Block data (6 bytes per block):
///     local_x:     u8  (0-199, position within chunk)
///     local_z:     u8  (0-199, position within chunk)
///     y:           i16 (height, little-endian)
///     material_id: u16 (Sosaia material ID, little-endian)

use crate::block_definitions::AIR;
use crate::material_ids::{self, MaterialId, MAT_AIR};
use crate::world_editor::common::{BlockStorage, WorldToModify};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Chunk dimensions in blocks (world units = meters at scale 1)
pub const CHUNK_SIZE: i32 = 200;

/// Magic bytes "SOSA"
const MAGIC: u32 = 0x534F5341;
const FORMAT_VERSION: u16 = 1;

/// Represents a single block in chunk export format
struct ExportBlock {
    local_x: u8,
    local_z: u8,
    y: i16,
    material_id: MaterialId,
}

/// City manifest for the Unity client
#[derive(serde::Serialize)]
pub struct CityManifest {
    pub city: String,
    pub bbox: BBoxInfo,
    pub chunk_size_meters: i32,
    pub chunks: Vec<ChunkInfo>,
    pub total_blocks: u64,
}

#[derive(serde::Serialize)]
pub struct BBoxInfo {
    pub min_lat: f64,
    pub min_lng: f64,
    pub max_lat: f64,
    pub max_lng: f64,
}

#[derive(serde::Serialize)]
pub struct ChunkInfo {
    pub x: i32,
    pub z: i32,
    pub file: String,
    pub block_count: u32,
    pub size_bytes: u64,
}

/// Export the world data as binary chunk files.
///
/// Iterates over all blocks in the WorldToModify, translates Minecraft block IDs
/// to Sosaia material IDs, splits by chunk, and writes binary files.
pub fn export_chunks(
    world: &WorldToModify,
    min_x: i32,
    max_x: i32,
    min_z: i32,
    max_z: i32,
    output_dir: &Path,
    city_name: &str,
    bbox: (f64, f64, f64, f64), // min_lat, min_lng, max_lat, max_lng
) -> Result<CityManifest, Box<dyn std::error::Error>> {
    println!("{} Exporting Sosaia chunks...", "[EXPORT]".bold().green());

    let chunks_dir = output_dir.join("chunks").join(city_name);
    fs::create_dir_all(&chunks_dir)?;

    println!(
        "  World bounds: x=[{}, {}] z=[{}, {}]",
        min_x, max_x, min_z, max_z
    );

    // Collect all non-air blocks, grouped by sosaia chunk coordinate
    let mut chunk_blocks: HashMap<(i32, i32), Vec<ExportBlock>> = HashMap::new();
    let mut total_blocks: u64 = 0;

    // Walk over arnis's internal data: regions → mc_chunks → sections → blocks
    for ((_rx, _rz), region) in &world.regions {
        for ((_cx, _cz), mc_chunk) in &region.chunks {
            for (&section_y, section) in &mc_chunk.sections {
                let base_y = (section_y as i32) * 16;

                // Compute absolute chunk origin for this MC chunk
                // MC chunk coords in RegionToModify are local (0-31),
                // but we stored them relative to the region.
                // However, the WorldToModify stores blocks by absolute world coords
                // via set_block, so we need to reconstruct absolute coords.
                //
                // Actually, arnis stores MC chunk coords as (chunk_x & 31, chunk_z & 31)
                // relative to region, and region coords as (chunk_x >> 5, chunk_z >> 5).
                // So absolute MC chunk coords = region * 32 + local_chunk
                let abs_mc_chunk_x = _rx * 32 + _cx;
                let abs_mc_chunk_z = _rz * 32 + _cz;
                let base_x = abs_mc_chunk_x * 16;
                let base_z = abs_mc_chunk_z * 16;

                // Iterate over all 4096 positions in this section
                for idx in 0..4096usize {
                    let block = section.storage.get(idx);
                    if block == AIR {
                        continue;
                    }

                    let mat_id = material_ids::block_to_material(&block);
                    if mat_id == MAT_AIR {
                        continue;
                    }

                    // Decode YZX index back to local coords
                    let local_y = (idx / 256) as u8;
                    let local_z = ((idx % 256) / 16) as u8;
                    let local_x = (idx % 16) as u8;

                    let world_x = base_x + local_x as i32;
                    let world_z = base_z + local_z as i32;
                    let world_y = base_y + local_y as i32;

                    // Determine which Sosaia chunk this block belongs to
                    let sosaia_cx = world_x.div_euclid(CHUNK_SIZE);
                    let sosaia_cz = world_z.div_euclid(CHUNK_SIZE);

                    let lx = world_x.rem_euclid(CHUNK_SIZE) as u8;
                    let lz = world_z.rem_euclid(CHUNK_SIZE) as u8;

                    chunk_blocks
                        .entry((sosaia_cx, sosaia_cz))
                        .or_default()
                        .push(ExportBlock {
                            local_x: lx,
                            local_z: lz,
                            y: world_y as i16,
                            material_id: mat_id,
                        });

                    total_blocks += 1;
                }
            }
        }
    }

    println!("  Total blocks to export: {}", total_blocks);
    println!("  Chunks to write: {}", chunk_blocks.len());

    // Write each chunk file
    let mut chunk_infos: Vec<ChunkInfo> = Vec::new();

    for ((cx, cz), blocks) in &chunk_blocks {
        let filename = format!("{}_{}.bin", cx, cz);
        let filepath = chunks_dir.join(&filename);

        let mut file = fs::File::create(&filepath)?;

        // Header (14 bytes)
        file.write_all(&MAGIC.to_le_bytes())?;
        file.write_all(&FORMAT_VERSION.to_le_bytes())?;
        file.write_all(&cx.to_le_bytes())?;
        file.write_all(&cz.to_le_bytes())?;

        // Block count (u32)
        let count = blocks.len() as u32;
        file.write_all(&count.to_le_bytes())?;

        // Block data (6 bytes per block)
        for block in blocks {
            file.write_all(&[block.local_x, block.local_z])?;
            file.write_all(&block.y.to_le_bytes())?;
            file.write_all(&block.material_id.to_le_bytes())?;
        }

        let file_size = filepath.metadata()?.len();
        chunk_infos.push(ChunkInfo {
            x: *cx,
            z: *cz,
            file: filename,
            block_count: blocks.len() as u32,
            size_bytes: file_size,
        });
    }

    chunk_infos.sort_by_key(|c| (c.x, c.z));

    let manifest = CityManifest {
        city: city_name.to_string(),
        bbox: BBoxInfo {
            min_lat: bbox.0,
            min_lng: bbox.1,
            max_lat: bbox.2,
            max_lng: bbox.3,
        },
        chunk_size_meters: CHUNK_SIZE,
        chunks: chunk_infos,
        total_blocks,
    };

    let manifest_path = chunks_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, &manifest_json)?;

    println!(
        "  {} Chunks exported to: {}",
        "Done!".bold().green(),
        chunks_dir.display()
    );

    Ok(manifest)
}
