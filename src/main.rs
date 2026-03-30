mod args;
mod block_definitions;
mod bresenham;
mod chunk_export;
mod clipping;
mod colors;
mod coordinate_system;
mod data_processing;
mod deterministic_rng;
mod element_processing;
mod elevation_data;
mod floodfill;
mod floodfill_cache;
mod greedy_mesher;
mod ground;
mod map_renderer;
mod map_transformation;
mod material_ids;
mod mesh_export;
mod osm_parser;
mod retrieve_data;
mod urban_ground;
mod world_editor;

// Mock progress/telemetry modules (no GUI)
mod progress {
    pub fn emit_gui_error(_message: &str) {}
    pub fn emit_gui_progress_update(_progress: f64, _message: &str) {}
    pub fn emit_map_preview_ready() {}
    pub fn emit_open_mcworld_file(_path: &str) {}
    pub fn is_running_with_gui() -> bool {
        false
    }
}

use args::Args;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

fn main() {
    // Configure thread pool
    floodfill_cache::configure_rayon_thread_pool(0.9);

    // Clean up old cached elevation tiles
    elevation_data::cleanup_old_cached_tiles();

    println!(
        "{}",
        r#"
   ███████  ██████  ███████  █████  ██  █████
   ██      ██    ██ ██      ██   ██ ██ ██   ██
   ███████ ██    ██ ███████ ███████ ██ ███████
        ██ ██    ██      ██ ██   ██ ██ ██   ██
   ███████  ██████  ███████ ██   ██ ██ ██   ██

          Preprocessing Pipeline v0.1.0
        "#
        .bright_cyan()
    );

    let args: Args = Args::parse();

    // Validate arguments
    let output_dir = args
        .path
        .clone()
        .unwrap_or_else(|| PathBuf::from("./output"));

    // Determine city name from bbox or use default
    let city_name = args
        .city_name
        .clone()
        .unwrap_or_else(|| "city".to_string());

    println!(
        "City: {}, Output: {}",
        city_name.bright_white().bold(),
        output_dir.display().to_string().bright_white().bold()
    );

    // Fetch data
    let raw_data = match &args.file {
        Some(file) => retrieve_data::fetch_data_from_file(file),
        None => retrieve_data::fetch_data_from_overpass(
            args.bbox,
            args.debug,
            args.downloader.as_str(),
            args.save_json_file.as_deref(),
        ),
    }
    .expect("Failed to fetch data");

    let mut ground = ground::generate_ground_data(&args);

    // Parse raw data
    let (mut parsed_elements, mut xzbbox) =
        osm_parser::parse_osm_data(raw_data, args.bbox, args.scale, args.debug);
    parsed_elements
        .sort_by_key(|element: &osm_parser::ProcessedElement| osm_parser::get_priority(element));

    // Transform map
    map_transformation::transform_map(&mut parsed_elements, &mut xzbbox, &mut ground);

    // Create a temporary directory for the world editor (it needs a path)
    let temp_dir = output_dir.join("_temp_world");
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");

    // Build generation options (Java format, we'll intercept the world data before save)
    let generation_options = data_processing::GenerationOptions {
        path: temp_dir.clone(),
        format: world_editor::WorldFormat::JavaAnvil,
        level_name: None,
        spawn_point: None,
    };

    // Generate world (this populates the WorldEditor's internal block data)
    println!("{}", "Generating world data...".bright_yellow());

    match data_processing::generate_world_for_export(
        parsed_elements,
        xzbbox,
        args.bbox,
        ground,
        &args,
        generation_options,
        &output_dir,
        &city_name,
    ) {
        Ok(manifest) => {
            println!(
                "\n{} Generated {} chunks with {} total blocks",
                "Success!".green().bold(),
                manifest.chunks.len(),
                manifest.total_blocks
            );
            println!(
                "Output: {}",
                output_dir
                    .join("chunks")
                    .join(&city_name)
                    .display()
                    .to_string()
                    .bright_white()
                    .bold()
            );
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }

    // Clean up temp directory
    let _ = std::fs::remove_dir_all(&temp_dir);
}
