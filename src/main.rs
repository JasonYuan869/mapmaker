use image::imageops::{overlay};
use image::{RgbImage, Rgb};
use std::process::exit;
use crate::color_list::{MINECRAFT_COLOR_MAP, RgbColorMap};
use std::fs::File;
use std::io::{Write, Read};
use nbt::{Blob, Map, Value, Error};
use std::{fs, io};
use std::path::Path;

mod color_list;

#[inline(always)]
fn ceil_div(dividend: u32, divisor: u32) -> u32 {
    (dividend + divisor - 1) / divisor
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), Error> {
    // Clear existing .dat files
    fs::remove_dir_all("./out/data")?;
    fs::create_dir("./out/data")?;

    let mut entries = fs::read_dir("./in")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    entries.sort();
    let mut index: u32 = 0;
    let mut out: (u32, u32) = (0, 0);
    for file in &entries {
        if file.extension().unwrap() == "jpg" || file.extension().unwrap() == "jpeg" || file.extension().unwrap() == "png" {
            print!("Processing image {}... ", index + 1);
            io::stdout().flush()?;
            out = make_nbt(file, index)?;
            index += 1;
            println!("Done.")
        }
    }

    let map_width = out.0;
    let map_height = out.1;

    println!("Generating datapack");
    match generate_datapack(index as i32, map_width as i32, map_height as i32) {
        Ok(_) => println!("All done!"),
        Err(e) => {
            println!("Error writing datapack.");
            println!("{}", e);
            exit(-1);
        }
    };
    pause();
    Ok(())
}

fn make_nbt(path: &Path, index: u32) -> Result<(u32, u32), Error> {
    let mut source = match image::open(path) {
        Ok(image) => image,
        Err(e) => {
            println!("The source path does not exist, or is not an image.");
            println!("{}", e);
            exit(-1);
        },
    };

    let im = match source.as_mut_rgb8() {
        Some(i) => i,
        None => {
            println!("There was an issue reading the image. Try converting to a JPG.");
            exit(-1);
        }
    };
    let width = im.width();
    let height = im.height();
    let map_width = ceil_div(width, 128);
    let map_width_pixels = map_width * 128;
    let map_height = ceil_div(height, 128);
    let map_height_pixels = map_height * 128;
    let map_count = map_width * map_height;

    let mut resized: RgbImage = RgbImage::new(map_width_pixels, map_height_pixels);

    // Resize the image buffer to fit a multiple of 128 by 128 pixels
    overlay(&mut resized, im,
            (map_width_pixels - width) / 2,
            (map_height_pixels - height) / 2);

    let pixel_array = convert_colors(&mut resized,
                                     &MINECRAFT_COLOR_MAP,
                                     map_width as usize);

    for i in 0..map_count {
        let start: usize = ((i % map_width) * 16384 + // H_Map
            (i / map_width) * (16384 * map_width)) as usize; // V_Map

        match generate_dat_file(&pixel_array[start..start + 16384], index * map_count + i) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing NBT data.");
                println!("{}", e);
                exit(-1);
            }
        }
    }
    Ok((map_width, map_height))
}

fn convert_colors(image: &mut RgbImage, map: &RgbColorMap, map_width: usize) -> Vec<i8> {
    // Returns the indices of the colour changes in a Minecraft map shape
    // The index of the array can be expressed as:
    // index = H_Pixel + 128 * V_Pixel + 16384 * H_Map + 16384 * map_width * V_Map
    // Each subset of 16384 entries represent one single map
    let mut indices: Vec<i8> = vec![0; image.pixels().len() as usize];
    for i in 0..image.pixels().len() {
        let x = i as u32 % image.width();
        let y = i as u32 / image.width();
        let pixel = image.get_pixel(x, y);
        let output = map.map_indices(pixel);
        let idx = output.0 as u8;
        // println!("{}", idx);
        indices[i % 128 +
            (i / (128 * map_width)) % 128 * 128 +
            (i / 128) % map_width * 16384 +
            i - (i % (16384*map_width))
            ] = idx as i8;

        // Propagate error for dithering
        let propagate_error = |error: [i8; 3], x: u32, y: u32, factor: f32| {
            let mut t: [u8; 3] = [0, 0, 0];
            for ((i, err), px) in t.iter_mut().zip(&error).zip(&image.get_pixel(x, y).0) {
                let f = *px as f32 / 256.0 + *err as f32 / 256.0 * factor;
                if f > 1.0 {
                    *i = 255;
                } else if f < 0.0 {
                    *i = 0;
                } else {
                    *i = (f * 256.0) as u8;
                }
            }
            Rgb(t)
        };

        if x > 1 && x < image.width() - 1 &&
            y < image.height() - 1 && output.1 != [0, 0, 0] {
            let dithering_array = [propagate_error(output.1, x + 1, y, 0.4375),
                propagate_error(output.1, x - 1, y + 1, 0.1875),
                propagate_error(output.1, x, y + 1, 0.3125),
                propagate_error(output.1, x + 1, y + 1, 0.0625)];
            image.put_pixel(x + 1, y, dithering_array[0]);
            image.put_pixel(x - 1, y + 1, dithering_array[1]);
            image.put_pixel(x, y + 1, dithering_array[2]);
            image.put_pixel(x + 1, y + 1, dithering_array[3]);
        }
    }
    indices
}

fn generate_dat_file(colors: &[i8], index: u32) -> Result<(), Error>{
    let mut filename = "out/data/map_".to_owned();
    filename.push_str(&*index.to_string());
    filename.push_str(".dat");

    let mut nbtfile = Blob::named("Data");
    nbtfile.insert("scale", 1_i8)?;
    nbtfile.insert("dimension", "minecraft:overworld")?;
    nbtfile.insert("trackingPosition", 0_i8)?;
    nbtfile.insert("unlimitedTracking", 0_i8)?;
    nbtfile.insert("xCenter", 100000_i32)?;
    nbtfile.insert("ZCenter", 100000_i32)?;
    nbtfile.insert("banners", Value::Compound(Map::new()))?;
    nbtfile.insert("frames", Value::Compound(Map::new()))?;
    nbtfile.insert("colors", colors)?;
    nbtfile.insert("DataVersion", 1343_i32)?;

    let mut file = File::create(filename).unwrap();
    nbtfile.to_gzip_writer(&mut file)
}

fn generate_datapack(frames: i32, map_width: i32, map_height: i32) -> Result<(), Box<dyn std::error::Error>> {
    let maps_per_frame = map_width * map_height;

    {
        let mut init = File::create("./out/datapacks/mapmaker/data/mapmaker/functions/init.mcfunction").unwrap();
        write!(&mut init, include_str!("init_commands.in"),
               maps_per_frame, frames, frames * maps_per_frame)?;

        for i in 0..maps_per_frame {
            write!(&mut init, include_str!("init_summon.in"),
                   0 - (i % map_width), 100 - (i / map_width), i, i, i, i)?;
        }
    }
    {
        let mut loop_ = File::create("out/datapacks/mapmaker/data/mapmaker/functions/loop.mcfunction").unwrap();
        write!(&mut loop_, include_str!("loop_commands.in"))?;
        for i in 0..maps_per_frame {
            write!(&mut loop_, include_str!("loop_scoreboard.in"),
                   i, i, i, i)?;
        }
    }
    {
        let mut restart = File::create("out/datapacks/mapmaker/data/mapmaker/functions/restart.mcfunction").unwrap();
        for i in 0..maps_per_frame{
            writeln!(&mut restart, "scoreboard players set @e[tag={}] map_num {}", i, i)?;
        }
    }
    Ok(())
}
