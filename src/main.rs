use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Context;

use rayon::prelude::*;

use crate::image_processor::Processor;
use crate::output_generator::Generator;

mod image_processor;
mod cli;
mod output_generator;

fn main() -> anyhow::Result<()> {
    let args = cli::run().with_context(|| "failed to launch CLI")?;
    let mut generator = Generator::new(&args.output_path, args.starting_index, args.top_left, args.direction)?;

    let mut entries = fs::read_dir(&args.input_path)
        .with_context(|| "failed to read input directory")?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if let Some("jpg") | Some("jpeg") | Some("png") = ext.to_str() {
                        return Some(entry.path());
                    }
                }
            }
            None
        })
        .collect::<Vec<PathBuf>>();

    if entries.is_empty() {
        anyhow::bail!("no images found in input directory");
    }

    // Sort images by name
    entries.sort_unstable();

    // Get the first image to initialize the processor with the dimensions
    let processor = Processor::new(&entries[0])?;
    generator.init_files(1, processor.map_columns as usize, processor.map_rows as usize)?;

    entries.par_iter().enumerate().for_each(|(frame, entry)| {
        println!("Processing frame {} of {}", frame + 1, entries.len());
        let image = processor.process_file(entry).unwrap();
        let maps = processor.convert_colors(image);
        maps.par_iter().enumerate().for_each(|(i, map)| {
            generator.generate_dat(map, i, frame).unwrap();
        });
    });
    generator.generate_idcounts()?;
    generator.generate_datapack()?;

    Ok(())
}