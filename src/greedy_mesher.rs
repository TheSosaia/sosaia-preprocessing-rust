/// Greedy mesher — port of the TypeScript greedy mesher to Rust.
///
/// Takes a 3D voxel grid of material IDs and produces vertex buffers
/// (positions, normals, colors, indices) grouped by material.
///
/// Based on Mikola Lysenko's algorithm:
/// https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/

use crate::material_ids::MaterialId;
use std::collections::HashMap;

/// LOD levels: (downsample factor, file suffix)
pub const LOD_LEVELS: &[(i32, &str)] = &[
    (1, ""),             // LOD0: full detail
    (2, ".lod1"),        // LOD1: 2x2x2 merge
    (4, ".lod2"),        // LOD2: 4x4x4 merge
];

/// AO brightness multipliers matching the web client
const AO_BRIGHTNESS: [f32; 4] = [0.4, 0.6, 0.8, 1.0];

/// Transparent material IDs (water + glass)
fn is_transparent(mat: u16) -> bool {
    mat == 6 || (mat >= 70 && mat <= 78)
}

fn is_opaque(mat: u16) -> bool {
    mat != 0 && !is_transparent(mat)
}

fn vertex_ao(side1: bool, side2: bool, corner: bool) -> u8 {
    if side1 && side2 {
        return 0;
    }
    3 - (side1 as u8) - (side2 as u8) - (corner as u8)
}

fn encode_mask(mat_id: u16, ao: [u8; 4]) -> i32 {
    ((mat_id as i32) << 8) | ((ao[0] as i32) << 6) | ((ao[1] as i32) << 4) | ((ao[2] as i32) << 2) | (ao[3] as i32)
}

fn decode_mat(encoded: i32) -> u16 {
    (encoded >> 8) as u16
}

fn decode_ao(encoded: i32) -> [u8; 4] {
    [
        ((encoded >> 6) & 3) as u8,
        ((encoded >> 4) & 3) as u8,
        ((encoded >> 2) & 3) as u8,
        (encoded & 3) as u8,
    ]
}

/// Input block for the mesher
#[derive(Clone)]
pub struct Block {
    pub local_x: u8,
    pub local_z: u8,
    pub y: i16,
    pub material_id: MaterialId,
}

/// Voxel grid for fast neighbor lookups
struct VoxelGrid {
    size_x: usize,
    size_y: usize,
    size_z: usize,
    min_y: i32,
    data: Vec<u16>,
}

impl VoxelGrid {
    fn new(size_x: usize, size_z: usize, min_y: i32, max_y: i32) -> Self {
        let size_y = (max_y - min_y + 1) as usize;
        Self {
            size_x,
            size_y,
            size_z,
            min_y,
            data: vec![0u16; size_x * size_y * size_z],
        }
    }

    fn populate(&mut self, blocks: &[Block]) {
        for b in blocks {
            if b.material_id == 0 {
                continue;
            }
            let y = (b.y as i32) - self.min_y;
            if y < 0 || y >= self.size_y as i32 {
                continue;
            }
            let x = b.local_x as usize;
            let z = b.local_z as usize;
            if x >= self.size_x || z >= self.size_z {
                continue;
            }
            let idx = y as usize * self.size_x * self.size_z + z * self.size_x + x;
            self.data[idx] = b.material_id;
        }
    }

    fn get(&self, x: i32, y: i32, z: i32) -> u16 {
        if x < 0 || x >= self.size_x as i32 || y < 0 || y >= self.size_y as i32 || z < 0 || z >= self.size_z as i32 {
            return 0;
        }
        self.data[self.index(x as usize, y as usize, z as usize)]
    }

    fn is_opaque_at(&self, x: i32, y: i32, z: i32) -> bool {
        is_opaque(self.get(x, y, z))
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        y * self.size_x * self.size_z + z * self.size_x + x
    }
}

/// Per-material mesh data
pub struct MaterialMesh {
    pub material_id: u16,
    pub positions: Vec<f32>,   // xyz, 3 per vertex
    pub normals: Vec<f32>,     // xyz, 3 per vertex
    pub colors: Vec<f32>,      // rgb, 3 per vertex (AO baked)
    pub indices: Vec<u32>,
    pub vertex_count: u32,
}

/// Result of meshing a chunk
pub struct MeshedChunk {
    pub meshes: Vec<MaterialMesh>,
}

/// Downsample blocks by merging NxNxN groups, picking the most common material.
/// Returns new blocks with adjusted coordinates and a new effective chunk size.
pub fn downsample_blocks(blocks: &[Block], chunk_size: i32, factor: i32) -> (Vec<Block>, i32) {
    if factor <= 1 {
        return (blocks.to_vec(), chunk_size);
    }

    // Find Y bounds
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for b in blocks {
        let y = b.y as i32;
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }
    if min_y > max_y {
        return (vec![], chunk_size / factor);
    }

    // Build a grid to count materials per downsampled cell
    let ds_size_x = (chunk_size + factor - 1) / factor;
    let ds_size_z = (chunk_size + factor - 1) / factor;
    let ds_min_y = min_y.div_euclid(factor);
    let ds_max_y = max_y.div_euclid(factor);
    let ds_size_y = (ds_max_y - ds_min_y + 1) as usize;

    // For each downsampled cell, track material counts
    let total_cells = ds_size_x as usize * ds_size_y * ds_size_z as usize;
    let mut cell_materials: Vec<HashMap<u16, u16>> = Vec::with_capacity(total_cells);
    cell_materials.resize_with(total_cells, HashMap::new);

    let ds_idx = |x: usize, y: usize, z: usize| -> usize {
        y * ds_size_x as usize * ds_size_z as usize + z * ds_size_x as usize + x
    };

    for b in blocks {
        if b.material_id == 0 { continue; }
        let dx = (b.local_x as i32 / factor) as usize;
        let dz = (b.local_z as i32 / factor) as usize;
        let dy = ((b.y as i32).div_euclid(factor) - ds_min_y) as usize;
        if dx < ds_size_x as usize && dz < ds_size_z as usize && dy < ds_size_y {
            let idx = ds_idx(dx, dy, dz);
            *cell_materials[idx].entry(b.material_id).or_insert(0) += 1;
        }
    }

    // Pick most common material per cell
    let mut result = Vec::new();
    for dy in 0..ds_size_y {
        for dz in 0..ds_size_z as usize {
            for dx in 0..ds_size_x as usize {
                let idx = ds_idx(dx, dy, dz);
                if let Some((&mat, _)) = cell_materials[idx].iter().max_by_key(|(_, &count)| count) {
                    result.push(Block {
                        local_x: dx as u8,
                        local_z: dz as u8,
                        y: (dy as i32 + ds_min_y) as i16,
                        material_id: mat,
                    });
                }
            }
        }
    }

    let new_chunk_size = ds_size_x.max(ds_size_z) as i32;
    (result, new_chunk_size)
}

/// Greedy mesh a chunk's blocks into vertex buffers.
pub fn greedy_mesh(
    blocks: &[Block],
    chunk_size: i32,
    world_offset_x: f32,
    world_offset_z: f32,
) -> MeshedChunk {
    // Find Y bounds
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for b in blocks {
        let y = b.y as i32;
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }
    if min_y > max_y {
        return MeshedChunk { meshes: vec![] };
    }

    // Build voxel grid
    let mut grid = VoxelGrid::new(chunk_size as usize, chunk_size as usize, min_y, max_y);
    grid.populate(blocks);

    let dims = [grid.size_x, grid.size_y, grid.size_z];
    let mut faces_by_material: HashMap<u16, MaterialMesh> = HashMap::new();

    // Compute AO for a face
    let compute_ao = |grid: &VoxelGrid, x: i32, y: i32, z: i32, d: usize, backface: bool| -> [u8; 4] {
        let u = (d + 1) % 3;
        let v = (d + 2) % 3;

        let mut pos = [x, y, z];
        if !backface {
            pos[d] += 1;
        }

        let solid = |du: i32, dv: i32| -> bool {
            let mut p = pos;
            p[u] += du;
            p[v] += dv;
            grid.is_opaque_at(p[0], p[1], p[2])
        };

        let s00 = solid(-1, 0);
        let s10 = solid(1, 0);
        let s01 = solid(0, -1);
        let s11 = solid(0, 1);
        let c00 = solid(-1, -1);
        let c10 = solid(1, -1);
        let c01 = solid(-1, 1);
        let c11 = solid(1, 1);

        [
            vertex_ao(s00, s01, c00),
            vertex_ao(s10, s01, c10),
            vertex_ao(s10, s11, c11),
            vertex_ao(s00, s11, c01),
        ]
    };

    // Sweep 3 axes
    for d in 0..3usize {
        let u = (d + 1) % 3;
        let v = (d + 2) % 3;
        let mut q = [0i32; 3];
        q[d] = 1;

        let mask_size = dims[u] * dims[v];
        let mut mask = vec![0i32; mask_size];
        let mut x = [0i32; 3];

        x[d] = -1;
        while x[d] < dims[d] as i32 {
            // Build mask
            let mut n = 0;
            x[v] = 0;
            while x[v] < dims[v] as i32 {
                x[u] = 0;
                while x[u] < dims[u] as i32 {
                    let a = if x[d] >= 0 { grid.get(x[0], x[1], x[2]) } else { 0 };
                    let b = if x[d] < dims[d] as i32 - 1 {
                        grid.get(x[0] + q[0], x[1] + q[1], x[2] + q[2])
                    } else {
                        0
                    };

                    let a_opaque = if x[d] >= 0 { grid.is_opaque_at(x[0], x[1], x[2]) } else { false };
                    let b_opaque = if x[d] < dims[d] as i32 - 1 {
                        grid.is_opaque_at(x[0] + q[0], x[1] + q[1], x[2] + q[2])
                    } else {
                        false
                    };

                    if a == b {
                        mask[n] = 0;
                    } else if a != 0 && !b_opaque {
                        let ao = compute_ao(&grid, x[0], x[1], x[2], d, false);
                        mask[n] = encode_mask(a, ao);
                    } else if b != 0 && !a_opaque {
                        let ao = compute_ao(&grid, x[0] + q[0], x[1] + q[1], x[2] + q[2], d, true);
                        mask[n] = -encode_mask(b, ao);
                    } else {
                        mask[n] = 0;
                    }
                    n += 1;
                    x[u] += 1;
                }
                x[v] += 1;
            }
            x[d] += 1;

            // Greedy merge
            n = 0;
            for j in 0..dims[v] {
                let mut i = 0;
                while i < dims[u] {
                    let mask_val = mask[n];
                    if mask_val != 0 {
                        // Expand width
                        let mut w = 1usize;
                        while i + w < dims[u] && mask[n + w] == mask_val {
                            w += 1;
                        }

                        // Expand height
                        let mut h = 1usize;
                        let mut done = false;
                        while j + h < dims[v] && !done {
                            for k in 0..w {
                                if mask[n + k + h * dims[u]] != mask_val {
                                    done = true;
                                    break;
                                }
                            }
                            if !done {
                                h += 1;
                            }
                        }

                        let backface = mask_val < 0;
                        let encoded = mask_val.unsigned_abs() as i32;
                        let mat_id = decode_mat(encoded);
                        let ao = decode_ao(encoded);

                        x[u] = i as i32;
                        x[v] = j as i32;

                        let mut du = [0i32; 3];
                        du[u] = w as i32;
                        let mut dv = [0i32; 3];
                        dv[v] = h as i32;

                        // World positions for 4 corners
                        let p0 = [
                            world_offset_x + x[0] as f32,
                            grid.min_y as f32 + x[1] as f32,
                            world_offset_z + x[2] as f32,
                        ];
                        let p1 = [p0[0] + du[0] as f32, p0[1] + du[1] as f32, p0[2] + du[2] as f32];
                        let p2 = [
                            p0[0] + du[0] as f32 + dv[0] as f32,
                            p0[1] + du[1] as f32 + dv[1] as f32,
                            p0[2] + du[2] as f32 + dv[2] as f32,
                        ];
                        let p3 = [p0[0] + dv[0] as f32, p0[1] + dv[1] as f32, p0[2] + dv[2] as f32];

                        let mut normal = [0.0f32; 3];
                        normal[d] = if backface { -1.0 } else { 1.0 };

                        let corners = if backface {
                            [p0, p3, p2, p1]
                        } else {
                            [p0, p1, p2, p3]
                        };

                        let data = faces_by_material.entry(mat_id).or_insert_with(|| MaterialMesh {
                            material_id: mat_id,
                            positions: Vec::new(),
                            normals: Vec::new(),
                            colors: Vec::new(),
                            indices: Vec::new(),
                            vertex_count: 0,
                        });
                        let base = data.vertex_count;

                        for c in 0..4 {
                            data.positions.extend_from_slice(&corners[c]);
                            data.normals.extend_from_slice(&normal);
                            let brightness = AO_BRIGHTNESS[ao[c] as usize];
                            data.colors.extend_from_slice(&[brightness, brightness, brightness]);
                        }

                        // AO-aware quad flip
                        if ao[0] as u16 + ao[2] as u16 > ao[1] as u16 + ao[3] as u16 {
                            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                        } else {
                            data.indices.extend_from_slice(&[base + 1, base + 2, base + 3, base + 1, base + 3, base]);
                        }
                        data.vertex_count += 4;

                        // Clear mask
                        for l in 0..h {
                            for k in 0..w {
                                mask[n + k + l * dims[u]] = 0;
                            }
                        }

                        i += w;
                        n += w;
                    } else {
                        i += 1;
                        n += 1;
                    }
                }
            }
        }
    }

    MeshedChunk {
        meshes: faces_by_material.into_values().collect(),
    }
}
