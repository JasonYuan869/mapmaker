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
    let mut image = processor.process_file(Path::new("1.jpg"))?;
    let z = processor.convert_colors(&mut image);
    image.save("out/1.jpg")?;

    generator.init_files(1, processor.maps_per_frame())?;
    for (i, map) in z.iter().enumerate() {
        generator.generate_dat(map, i)?;
    }
    generator.generate_idcounts()?;


    Ok(())
}