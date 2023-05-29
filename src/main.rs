use std::fs;
use std::path::{PathBuf};
use anyhow::Context;
use indicatif::ParallelProgressIterator;

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

    // Time the conversion process
    println!("Starting conversion process...");
    let start = std::time::Instant::now();

    generator.init_files(entries.len(), processor.map_columns as usize, processor.map_rows as usize)?;

    entries.par_iter().enumerate().progress_count(entries.len() as u64).for_each(|(frame, entry)| {
        let image = processor.process_file(entry).unwrap();
        let maps = processor.convert_colors(image);

        maps.par_iter().enumerate().for_each(|(i, map)| {
            generator.generate_dat(map, i, frame).unwrap();
        });
    });

    let duration = start.elapsed();
    println!("Finished in {}.{:03} seconds", duration.as_secs(), duration.subsec_millis());

    generator.generate_idcounts()?;
    generator.generate_datapack()?;

    Ok(())
}