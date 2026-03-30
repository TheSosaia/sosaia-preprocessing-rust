/// Mesh export — writes precomputed greedy-meshed geometry as binary files.
///
/// Format (.mesh.bin):
///   Header (10 bytes):
///     magic:       u32 LE = 0x534F534D ("SOSM")
///     version:     u16 LE = 1
///     chunk_x:     i16 LE  (global chunk coord)
///     chunk_z:     i16 LE
///
///   Material count: u16 LE
///
///   For each material:
///     material_id:    u16 LE
///     vertex_count:   u32 LE
///     index_count:    u32 LE
///     positions:      [f32 LE; vertex_count * 3]
///     normals:        [f32 LE; vertex_count * 3]
///     colors:         [f32 LE; vertex_count * 3]  (AO baked in)
///     indices:        [u32 LE; index_count]

use crate::greedy_mesher::MeshedChunk;
use std::fs;
use std::io::Write;
use std::path::Path;

const MESH_MAGIC: u32 = 0x534F534D; // "SOSM"
const MESH_VERSION: u16 = 1;

/// Write a meshed chunk to a binary file.
pub fn write_mesh_file(
    meshed: &MeshedChunk,
    chunk_x: i16,
    chunk_z: i16,
    output_path: &Path,
) -> Result<u64, Box<dyn std::error::Error>> {
    let mut file = fs::File::create(output_path)?;

    // Header
    file.write_all(&MESH_MAGIC.to_le_bytes())?;
    file.write_all(&MESH_VERSION.to_le_bytes())?;
    file.write_all(&chunk_x.to_le_bytes())?;
    file.write_all(&chunk_z.to_le_bytes())?;

    // Filter out empty meshes
    let non_empty: Vec<_> = meshed.meshes.iter().filter(|m| m.vertex_count > 0).collect();

    // Material count
    file.write_all(&(non_empty.len() as u16).to_le_bytes())?;

    for mesh in &non_empty {
        file.write_all(&mesh.material_id.to_le_bytes())?;
        file.write_all(&mesh.vertex_count.to_le_bytes())?;
        file.write_all(&(mesh.indices.len() as u32).to_le_bytes())?;

        // Positions
        for &val in &mesh.positions {
            file.write_all(&val.to_le_bytes())?;
        }
        // Normals
        for &val in &mesh.normals {
            file.write_all(&val.to_le_bytes())?;
        }
        // Colors (AO)
        for &val in &mesh.colors {
            file.write_all(&val.to_le_bytes())?;
        }
        // Indices
        for &val in &mesh.indices {
            file.write_all(&val.to_le_bytes())?;
        }
    }

    let size = output_path.metadata()?.len();
    Ok(size)
}

/// Manifest entry for a mesh file
#[derive(serde::Serialize)]
pub struct MeshChunkInfo {
    pub x: i32,
    pub z: i32,
    pub file: String,
    pub vertex_count: u32,
    pub index_count: u32,
    pub material_count: u16,
    pub size_bytes: u64,
}

/// City manifest for precomputed meshes
#[derive(serde::Serialize)]
pub struct MeshManifest {
    pub city: String,
    pub bbox: MeshBBoxInfo,
    pub chunk_size_meters: i32,
    pub chunks: Vec<MeshChunkInfo>,
    pub format: String, // "mesh_v1"
}

#[derive(serde::Serialize)]
pub struct MeshBBoxInfo {
    pub min_lat: f64,
    pub min_lng: f64,
    pub max_lat: f64,
    pub max_lng: f64,
}
