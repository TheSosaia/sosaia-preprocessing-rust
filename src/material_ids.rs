/// Sosaia Material ID system.
///
/// Maps Minecraft block IDs (from arnis) to Sosaia material IDs (uint16).
/// Unity client has a MaterialRegistry that maps these IDs to textures/shaders.
///
/// ID allocation:
///   0       = air (skip rendering)
///   1-9     = terrain
///   10-19   = roads
///   20-49   = building walls
///   50-69   = roofs
///   70-79   = windows/glass
///   80-89   = details (doors, fences, rails)
///   90-99   = nature (trees, plants)
///   100-109 = interior (furniture)
///   110+    = reserved

use crate::block_definitions::Block;

pub type MaterialId = u16;

pub const MAT_AIR: MaterialId = 0;

// Terrain (1-9)
pub const MAT_GRASS: MaterialId = 1;
pub const MAT_DIRT: MaterialId = 2;
pub const MAT_SAND: MaterialId = 3;
pub const MAT_STONE: MaterialId = 4;
pub const MAT_GRAVEL: MaterialId = 5;
pub const MAT_WATER: MaterialId = 6;
pub const MAT_SNOW: MaterialId = 7;
pub const MAT_MUD: MaterialId = 8;
pub const MAT_FARMLAND: MaterialId = 9;

// Roads (10-19)
pub const MAT_ASPHALT: MaterialId = 10;
pub const MAT_CONCRETE_LIGHT: MaterialId = 11;
pub const MAT_COBBLESTONE: MaterialId = 12;
pub const MAT_DIRT_PATH: MaterialId = 13;
pub const MAT_BRICK_ROAD: MaterialId = 14;
pub const MAT_STONE_SLAB: MaterialId = 15;

// Building walls (20-49)
pub const MAT_BRICK_RED: MaterialId = 20;
pub const MAT_BRICK_DARK: MaterialId = 21;
pub const MAT_STONE_BRICKS: MaterialId = 22;
pub const MAT_CONCRETE_WHITE: MaterialId = 23;
pub const MAT_CONCRETE_GRAY: MaterialId = 24;
pub const MAT_CONCRETE_DARK: MaterialId = 25;
pub const MAT_SANDSTONE: MaterialId = 26;
pub const MAT_SANDSTONE_SMOOTH: MaterialId = 27;
pub const MAT_QUARTZ: MaterialId = 28;
pub const MAT_WOOD_OAK: MaterialId = 29;
pub const MAT_WOOD_DARK: MaterialId = 30;
pub const MAT_WOOD_SPRUCE: MaterialId = 31;
pub const MAT_WOOD_BIRCH: MaterialId = 32;
pub const MAT_WOOD_ACACIA: MaterialId = 33;
pub const MAT_ANDESITE: MaterialId = 34;
pub const MAT_GRANITE: MaterialId = 35;
pub const MAT_DIORITE: MaterialId = 36;
pub const MAT_DEEPSLATE: MaterialId = 37;
pub const MAT_BLACKSTONE: MaterialId = 38;
pub const MAT_TERRACOTTA: MaterialId = 39;
pub const MAT_TERRACOTTA_WHITE: MaterialId = 40;
pub const MAT_TERRACOTTA_BROWN: MaterialId = 41;
pub const MAT_TERRACOTTA_GRAY: MaterialId = 42;
pub const MAT_TERRACOTTA_LIGHT_BLUE: MaterialId = 43;
pub const MAT_CONCRETE_BROWN: MaterialId = 44;
pub const MAT_MUD_BRICKS: MaterialId = 45;
pub const MAT_NETHER_BRICK: MaterialId = 46;
pub const MAT_PRISMARINE: MaterialId = 47;
pub const MAT_IRON_BLOCK: MaterialId = 48;
pub const MAT_CONCRETE_ORANGE: MaterialId = 49;

// Roofs (50-69)
pub const MAT_ROOF_STONE_BRICK: MaterialId = 50;
pub const MAT_ROOF_BRICK: MaterialId = 51;
pub const MAT_ROOF_DARK: MaterialId = 52;
pub const MAT_ROOF_SANDSTONE: MaterialId = 53;
pub const MAT_ROOF_WOOD: MaterialId = 54;
pub const MAT_ROOF_QUARTZ: MaterialId = 55;
pub const MAT_ROOF_FLAT_WHITE: MaterialId = 56;
pub const MAT_ROOF_FLAT_GRAY: MaterialId = 57;

// Windows/glass (70-79)
pub const MAT_GLASS_CLEAR: MaterialId = 70;
pub const MAT_GLASS_WHITE: MaterialId = 71;
pub const MAT_GLASS_GRAY: MaterialId = 72;
pub const MAT_GLASS_LIGHT_GRAY: MaterialId = 73;
pub const MAT_GLASS_BROWN: MaterialId = 74;
pub const MAT_GLASS_BLUE: MaterialId = 75;
pub const MAT_GLASS_CYAN: MaterialId = 76;
pub const MAT_GLASS_TINTED: MaterialId = 77;
pub const MAT_GLASS_PANE: MaterialId = 78;

// Details (80-89)
pub const MAT_DOOR_OAK: MaterialId = 80;
pub const MAT_DOOR_DARK: MaterialId = 81;
pub const MAT_DOOR_SPRUCE: MaterialId = 82;
pub const MAT_FENCE_OAK: MaterialId = 83;
pub const MAT_RAIL: MaterialId = 84;
pub const MAT_LADDER: MaterialId = 85;
pub const MAT_IRON_BARS: MaterialId = 86;
pub const MAT_WALL_STONE: MaterialId = 87;
pub const MAT_WALL_BRICK: MaterialId = 88;
pub const MAT_WALL_COBBLE: MaterialId = 89;

// Nature (90-99)
pub const MAT_LOG_OAK: MaterialId = 90;
pub const MAT_LOG_DARK_OAK: MaterialId = 91;
pub const MAT_LOG_BIRCH: MaterialId = 92;
pub const MAT_LOG_SPRUCE: MaterialId = 93;
pub const MAT_LOG_ACACIA: MaterialId = 94;
pub const MAT_LOG_JUNGLE: MaterialId = 95;
pub const MAT_LEAVES_OAK: MaterialId = 96;
pub const MAT_LEAVES_DARK_OAK: MaterialId = 97;
pub const MAT_LEAVES_BIRCH: MaterialId = 98;
pub const MAT_LEAVES_SPRUCE: MaterialId = 99;

// Interior (100-109)
pub const MAT_BOOKSHELF: MaterialId = 100;
pub const MAT_CRAFTING_TABLE: MaterialId = 101;
pub const MAT_FURNACE: MaterialId = 102;
pub const MAT_CARPET_WHITE: MaterialId = 103;
pub const MAT_CARPET_RED: MaterialId = 104;
pub const MAT_BED: MaterialId = 105;
pub const MAT_CHEST: MaterialId = 106;
pub const MAT_BARREL: MaterialId = 107;

// Misc (110+)
pub const MAT_BEDROCK: MaterialId = 110;
pub const MAT_GLOWSTONE: MaterialId = 111;
pub const MAT_FLOWER_RED: MaterialId = 112;
pub const MAT_FLOWER_YELLOW: MaterialId = 113;
pub const MAT_FLOWER_BLUE: MaterialId = 114;
pub const MAT_SHORT_GRASS: MaterialId = 115;
pub const MAT_TALL_GRASS: MaterialId = 116;
pub const MAT_DEAD_BUSH: MaterialId = 117;
pub const MAT_ICE: MaterialId = 118;
pub const MAT_PACKED_ICE: MaterialId = 119;
pub const MAT_COARSE_DIRT: MaterialId = 120;
pub const MAT_PODZOL: MaterialId = 121;
pub const MAT_HAY_BALE: MaterialId = 122;
pub const MAT_SCAFFOLDING: MaterialId = 123;
pub const MAT_COBWEB: MaterialId = 124;
pub const MAT_MOSS: MaterialId = 125;
pub const MAT_LEAVES_JUNGLE: MaterialId = 126;
pub const MAT_LEAVES_ACACIA: MaterialId = 127;
pub const MAT_FERN: MaterialId = 128;
pub const MAT_SPONGE: MaterialId = 129;
pub const MAT_CLAY: MaterialId = 130;
pub const MAT_SIGN: MaterialId = 131;

/// Translate a Minecraft Block (arnis ID) to a Sosaia MaterialId.
/// This runs at export time so element_processing code stays unchanged.
pub fn block_to_material(block: &Block) -> MaterialId {
    match block.id() {
        1 => MAT_AIR,                    // air
        28 => MAT_GRASS,                 // grass_block
        22 => MAT_DIRT,                  // dirt
        71 => MAT_SAND,                  // sand
        84 => MAT_STONE,                // stone
        30 => MAT_GRAVEL,               // gravel
        87 => MAT_WATER,                // water
        111 => MAT_SNOW,                // snow_block
        112 => MAT_SNOW,                // snow layer
        135 => MAT_MUD,                 // mud
        24 => MAT_FARMLAND,             // farmland

        // Roads
        5 => MAT_ASPHALT,               // black_concrete
        42 | 31 => MAT_CONCRETE_LIGHT,  // light_gray_concrete, gray_concrete
        13 => MAT_COBBLESTONE,           // cobblestone
        132 => MAT_DIRT_PATH,            // dirt_path
        81 | 82 => MAT_STONE_SLAB,      // stone_block_slab, stone_brick_slab

        // Building walls
        9 => MAT_BRICK_RED,              // bricks
        46 => MAT_NETHER_BRICK,          // nether_bricks
        83 => MAT_STONE_BRICKS,          // stone_bricks
        11 => MAT_STONE_BRICKS,          // chiseled_stone_bricks
        15 => MAT_STONE_BRICKS,          // cracked_stone_bricks
        88 => MAT_CONCRETE_WHITE,        // white_concrete
        174 => MAT_CONCRETE_BROWN,       // brown_concrete
        72 => MAT_SANDSTONE,             // sandstone
        17 => MAT_SANDSTONE,             // cut_sandstone
        76 => MAT_SANDSTONE_SMOOTH,      // smooth_sandstone
        57 | 65 => MAT_QUARTZ,           // quartz_block, quartz_bricks
        51 => MAT_WOOD_OAK,              // oak_planks
        19 => MAT_WOOD_DARK,             // dark_oak_planks
        80 => MAT_WOOD_SPRUCE,           // spruce_planks
        0 => MAT_WOOD_ACACIA,            // acacia_planks
        38 => MAT_WOOD_OAK,              // jungle_planks
        86 => MAT_WOOD_OAK,              // warped_planks
        16 => MAT_WOOD_OAK,              // crimson_planks
        2 | 55 => MAT_ANDESITE,          // andesite, polished_andesite
        27 | 61 => MAT_GRANITE,          // granite, polished_granite
        21 | 60 => MAT_DIORITE,          // diorite, polished_diorite
        20 | 59 => MAT_DEEPSLATE,        // deepslate_bricks, polished_deepslate
        6 | 58 | 14 => MAT_BLACKSTONE,   // blackstone, polished_blackstone, polished_blackstone_bricks
        85 => MAT_TERRACOTTA,            // terracotta
        91 => MAT_TERRACOTTA_WHITE,      // white_terracotta
        176 => MAT_TERRACOTTA_BROWN,     // brown_terracotta
        32 | 216 => MAT_TERRACOTTA_GRAY, // gray_terracotta, light_gray_terracotta
        41 => MAT_TERRACOTTA_LIGHT_BLUE, // light_blue_terracotta
        45 => MAT_MUD_BRICKS,            // mud_bricks
        37 => MAT_IRON_BLOCK,            // iron_block
        62 => MAT_PRISMARINE,            // prismarine
        77 => MAT_CONCRETE_LIGHT,        // smooth_stone
        23 => MAT_SANDSTONE,             // end_stone_bricks
        204 => MAT_CONCRETE_ORANGE,      // orange_concrete
        47 => MAT_BLACKSTONE,            // netherite_block

        // Colored concrete
        93 => MAT_CONCRETE_WHITE,        // yellow_concrete
        96 => MAT_CONCRETE_LIGHT,        // lime_concrete
        18 => MAT_CONCRETE_GRAY,         // cyan_concrete
        40 => MAT_CONCRETE_LIGHT,        // light_blue_concrete
        98 => MAT_CONCRETE_GRAY,         // blue_concrete
        99 => MAT_CONCRETE_GRAY,         // purple_concrete
        100 => MAT_BRICK_RED,            // red_concrete
        101 => MAT_CONCRETE_LIGHT,       // magenta_concrete

        // Colored terracotta
        53 => MAT_TERRACOTTA,            // orange_terracotta
        69 => MAT_TERRACOTTA,            // red_terracotta
        8 => MAT_TERRACOTTA,             // blue_terracotta
        33 => MAT_TERRACOTTA,            // green_terracotta
        175 => MAT_TERRACOTTA,           // black_terracotta
        104 => MAT_TERRACOTTA,           // yellow_terracotta

        // Colored wool
        92 => MAT_CONCRETE_WHITE,        // white_wool
        95 => MAT_CONCRETE_WHITE,        // yellow_wool
        70 => MAT_BRICK_RED,             // red_wool
        34 => MAT_CONCRETE_LIGHT,        // green_wool
        97 => MAT_CONCRETE_GRAY,         // cyan_wool
        102 => MAT_CONCRETE_BROWN,       // brown_wool
        205 => MAT_CONCRETE_ORANGE,      // orange_wool
        206 => MAT_CONCRETE_GRAY,        // blue_wool

        // Stairs → map to their base material for roofs
        177 => MAT_ROOF_STONE_BRICK,     // stone_brick_stairs
        178 => MAT_ROOF_SANDSTONE,       // mud_brick_stairs
        179 => MAT_ROOF_DARK,            // polished_blackstone_brick_stairs
        180 => MAT_ROOF_BRICK,           // brick_stairs
        181 => MAT_ROOF_STONE_BRICK,     // polished_granite_stairs
        182 => MAT_ROOF_SANDSTONE,       // end_stone_brick_stairs
        183 => MAT_ROOF_STONE_BRICK,     // polished_diorite_stairs
        184 => MAT_ROOF_SANDSTONE,       // smooth_sandstone_stairs
        185 => MAT_ROOF_QUARTZ,          // quartz_stairs
        186 => MAT_ROOF_STONE_BRICK,     // polished_andesite_stairs
        187 => MAT_ROOF_DARK,            // nether_brick_stairs
        144 => MAT_ROOF_WOOD,            // oak_stairs

        // Glass
        25 => MAT_GLASS_CLEAR,           // glass
        90 => MAT_GLASS_WHITE,           // white_stained_glass
        169 => MAT_GLASS_GRAY,           // gray_stained_glass
        170 => MAT_GLASS_LIGHT_GRAY,     // light_gray_stained_glass
        171 => MAT_GLASS_BROWN,          // brown_stained_glass
        172 => MAT_GLASS_TINTED,         // tinted_glass
        226 => MAT_GLASS_CYAN,           // cyan_stained_glass
        227 => MAT_GLASS_BLUE,           // blue_stained_glass
        228 => MAT_GLASS_BLUE,           // light_blue_stained_glass
        215 => MAT_GLASS_PANE,           // glass_pane
        230 => MAT_GLASS_CLEAR,          // red_stained_glass
        231 => MAT_GLASS_CLEAR,          // yellow_stained_glass
        232 => MAT_GLASS_CLEAR,          // purple_stained_glass
        233 => MAT_GLASS_CLEAR,          // orange_stained_glass
        234 => MAT_GLASS_CLEAR,          // magenta_stained_glass

        // Doors
        159 | 218 => MAT_DOOR_OAK,       // oak_door
        106 | 107 => MAT_DOOR_DARK,      // dark_oak_door
        212 | 213 => MAT_DOOR_SPRUCE,    // spruce_door

        // Details
        48 => MAT_FENCE_OAK,             // oak_fence
        66 | 116..=125 => MAT_RAIL,      // rail variants
        39 => MAT_LADDER,                // ladder
        36 => MAT_IRON_BARS,             // iron_bars
        114 | 115 => MAT_WALL_STONE,     // andesite_wall, stone_brick_wall
        12 => MAT_WALL_COBBLE,           // cobblestone_wall
        208 => MAT_WALL_BRICK,           // brick_wall
        173 | 236..=239 => MAT_DOOR_OAK, // oak_trapdoor variants
        241 => MAT_DOOR_DARK,            // dark_oak_trapdoor
        242 => MAT_DOOR_SPRUCE,          // spruce_trapdoor
        243 => MAT_DOOR_OAK,             // birch_trapdoor
        199 | 210 | 211 => MAT_IRON_BARS, // chain variants
        200 | 201 => MAT_IRON_BARS,      // end_rod, lightning_rod
        214 => MAT_STONE_SLAB,           // smooth_stone_slab
        217 => MAT_WOOD_OAK,             // oak_slab_top
        240 => MAT_QUARTZ,               // quartz_slab_top
        244 => MAT_MUD_BRICKS,           // mud_brick_slab
        245 => MAT_BRICK_RED,            // brick_slab

        // Nature - logs
        50 => MAT_LOG_OAK,              // oak_log
        219 => MAT_LOG_DARK_OAK,        // dark_oak_log
        4 => MAT_LOG_BIRCH,             // birch_log
        79 => MAT_LOG_SPRUCE,           // spruce_log
        223 => MAT_LOG_ACACIA,          // acacia_log
        221 => MAT_LOG_JUNGLE,          // jungle_log

        // Nature - leaves
        49 => MAT_LEAVES_OAK,           // oak_leaves
        220 => MAT_LEAVES_DARK_OAK,     // dark_oak_leaves
        3 => MAT_LEAVES_BIRCH,          // birch_leaves
        225 => MAT_LEAVES_SPRUCE,       // spruce_leaves
        222 => MAT_LEAVES_JUNGLE,       // jungle_leaves
        224 => MAT_LEAVES_ACACIA,       // acacia_leaves

        // Nature - plants
        29 => MAT_SHORT_GRASS,           // short_grass
        137 | 138 => MAT_TALL_GRASS,     // tall_grass
        136 => MAT_DEAD_BUSH,            // dead_bush
        67 => MAT_FLOWER_RED,            // poppy
        94 => MAT_FLOWER_YELLOW,         // dandelion
        7 | 89 => MAT_FLOWER_BLUE,       // blue_orchid, azure_bluet
        189 => MAT_FERN,                 // fern
        197 | 198 => MAT_FERN,           // large_fern

        // Interior
        142 => MAT_BOOKSHELF,            // bookshelf
        191..=194 => MAT_BOOKSHELF,      // chiseled_bookshelf variants
        139 => MAT_CRAFTING_TABLE,       // crafting_table
        140 => MAT_FURNACE,              // furnace
        141 => MAT_CARPET_WHITE,         // white_carpet
        156 => MAT_CARPET_RED,           // red_carpet
        161..=168 => MAT_BED,            // bed variants
        155 => MAT_CHEST,                // chest
        188 => MAT_BARREL,               // barrel
        143 => MAT_WOOD_OAK,             // oak_pressure_plate
        157 | 195 | 196 => MAT_IRON_BLOCK, // anvil variants
        158 => MAT_WOOD_OAK,             // note_block
        160 => MAT_CRAFTING_TABLE,       // brewing_stand
        235 | 246..=248 => MAT_FLOWER_RED, // flower_pot variants

        // Misc
        110 => MAT_BEDROCK,             // bedrock
        26 => MAT_GLOWSTONE,            // glowstone
        133 => MAT_ICE,                 // ice
        134 => MAT_PACKED_ICE,          // packed_ice
        126 => MAT_COARSE_DIRT,         // coarse_dirt
        54 => MAT_PODZOL,              // podzol
        35 => MAT_HAY_BALE,            // hay_block
        73 => MAT_SCAFFOLDING,         // scaffolding
        190 => MAT_COBWEB,             // cobweb
        43 | 44 => MAT_MOSS,           // moss_block, mossy_cobblestone
        131 => MAT_CLAY,               // clay
        78 => MAT_SPONGE,              // sponge
        113 => MAT_SIGN,               // oak_sign
        229 => MAT_GLASS_CLEAR,        // daylight_detector
        203 => MAT_GLOWSTONE,          // sea_lantern
        202 => MAT_IRON_BLOCK,         // gold_block
        209 => MAT_BRICK_RED,          // redstone_block
        103 => MAT_CONCRETE_GRAY,      // oxidized_copper

        // Ores (map to stone variants)
        127..=130 => MAT_STONE,         // iron_ore, coal_ore, gold_ore, copper_ore

        // Crops
        105 | 108 | 109 => MAT_FARMLAND, // carrots, potatoes, wheat

        // Misc blocks
        10 => MAT_STONE,                // cauldron
        56 => MAT_BLACKSTONE,           // polished_basalt
        63 | 64 => MAT_QUARTZ,          // purpur_block, purpur_pillar
        74 => MAT_QUARTZ,               // smooth_quartz
        75 => MAT_SANDSTONE,            // smooth_red_sandstone
        68 => MAT_NETHER_BRICK,         // red_nether_bricks

        _ => MAT_STONE,                 // fallback
    }
}
