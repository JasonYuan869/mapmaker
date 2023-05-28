use std::fs;
use std::fs::File;
use std::path::Path;
use crate::image_processor::Processor;
use crate::output_generator::Generator;

mod image_processor;
mod cli;
mod output_generator;

fn main() -> anyhow::Result<()> {
    let mut processor = Processor::default();
    let mut generator = Generator::new(Path::new("out"), 0)?;
    let image = processor.process_file(Path::new("1.png"))?;
    let z = processor.convert_colors(image);

    generator.init_files(1, processor.maps_per_frame())?;
    generator.generate_dat(&z[0], 0)?;


    Ok(())
}