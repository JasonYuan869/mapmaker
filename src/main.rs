use crate::color_list::{RgbColorMap, MINECRAFT_COLOR_MAP};
use image::imageops::overlay;
use image::{Rgb, RgbImage};
use nbt::{Blob, Error, Map, Value};
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

mod color_list;

#[inline(always)]
fn ceil_div(dividend: u32, divisor: u32) -> u32 {
    (dividend + divisor - 1) / divisor
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const DIRECTION_NORTH: i8 = 2;
    const DIRECTION_EAST: i8 = 5;
    const DIRECTION_SOUTH: i8 = 3;
    const DIRECTION_WEST: i8 = 4;
    // let threads = num_cpus::get();

    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!(
            "Usage: {} <X <Y> <Z> <Facing Direction> <Starting Index>",
            args[0]
        );
        eprintln!("Example: {} 0 100 0 east 2", args[0]);
        std::process::exit(1);
    }

    let location: [i64; 3] = match (
        i64::from_str(&args[1]),
        i64::from_str(&args[2]),
        i64::from_str(&args[3]),
    ) {
        (Ok(x), Ok(y), Ok(z)) => [x, y, z],
        _ => {
            eprintln!("Invalid coordinates. Ensure that they are all integer values.");
            std::process::exit(1);
        }
    };

    let facing: i8 = match args[4].to_lowercase().as_str() {
        "north" => DIRECTION_NORTH,
        "east" => DIRECTION_EAST,
        "south" => DIRECTION_SOUTH,
        "west" => DIRECTION_WEST,
        _ => {
            eprintln!(
                "Invalid direction. \
            Valid values are: north, east, south, west."
            );
            std::process::exit(1);
        }
    };

    let starting_index: i32 = match i32::from_str(&args[5]) {
        Ok(i) => i,
        _ => {
            eprintln!(
                "Invalid index. \
            Enter the starting map index that the program should begin with."
            );
            std::process::exit(1);
        }
    };

    // Clear existing .dat files
    if Path::new("./out/data").exists() {
        fs::remove_dir_all("./out/data")?;
    }
    fs::create_dir("./out/data")?;

    let mut entries = fs::read_dir("./in")?
        .filter_map(|entry| match entry {
            Ok(entry) => match entry.path().extension() {
                Some(ext) => match ext.to_str() {
                    Some("jpg") | Some("jpeg") | Some("png") => Some(entry.path()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<PathBuf>>();

    if entries.len() == 0 {
        eprintln!(
            "There are no valid files in the in/ directory. \
        Valid files are jpg, jpeg, and png."
        );
        std::process::exit(1);
    }

    let mut index = starting_index;
    entries.sort_unstable();
    let mut out: (u32, u32) = (0, 0);
    for file in &entries {
        print!("Processing image {}... ", index + 1 - starting_index);
        io::stdout().flush()?;
        out = make_nbt(file, (index - starting_index) as u32, starting_index as u32)?;
        index += 1;
        println!("Done.")
    }

    let map_width = out.0;
    let map_height = out.1;

    println!("Generating datapack");
    match generate_datapack(
        starting_index,
        index - starting_index,
        map_width as i32,
        map_height as i32,
        location,
        facing,
    ) {
        Ok(_) => println!("All done!"),
        Err(e) => {
            println!("Error writing datapack.");
            println!("{}", e);
            std::process::exit(1);
        }
    };
    Ok(())
}

fn make_nbt(
    path: &Path,
    index: u32,
    starting_index: u32,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut source = match image::open(path) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("The source path does not exist, or is not an image.");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let im = match source.as_mut_rgb8() {
        Some(i) => i,
        None => {
            eprintln!("There was an issue reading the image. Try converting to a JPG.");
            std::process::exit(1);
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
    overlay(
        &mut resized,
        im,
        (map_width_pixels - width) / 2,
        (map_height_pixels - height) / 2,
    );

    let pixel_array = convert_colors(&mut resized, &MINECRAFT_COLOR_MAP, map_width as usize);

    for i in 0..map_count {
        let start: usize = ((i % map_width) * 16384 + // H_Map
            (i / map_width) * (16384 * map_width)) as usize; // V_Map

        match generate_dat_file(
            &pixel_array[start..start + 16384],
            index * map_count + i + starting_index,
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing NBT data.");
                println!("{}", e);
                std::process::exit(1)
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
        indices[i % 128
            + (i / (128 * map_width)) % 128 * 128
            + (i / 128) % map_width * 16384
            + i
            - (i % (16384 * map_width))] = idx as i8;

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

        if x > 1 && x < image.width() - 1 && y < image.height() - 1 && output.1 != [0, 0, 0] {
            let dithering_array = [
                propagate_error(output.1, x + 1, y, 0.4375),
                propagate_error(output.1, x - 1, y + 1, 0.1875),
                propagate_error(output.1, x, y + 1, 0.3125),
                propagate_error(output.1, x + 1, y + 1, 0.0625),
            ];
            image.put_pixel(x + 1, y, dithering_array[0]);
            image.put_pixel(x - 1, y + 1, dithering_array[1]);
            image.put_pixel(x, y + 1, dithering_array[2]);
            image.put_pixel(x + 1, y + 1, dithering_array[3]);
        }
    }
    indices
}

fn generate_dat_file(colors: &[i8], index: u32) -> Result<(), Error> {
    let mut filename = "out/data/map_".to_owned();
    filename.push_str(&*index.to_string());
    filename.push_str(".dat");

    // Used for the inner "Data" compound
    let mut data: Map<String, Value> = Map::new();
    data.insert("scale".to_string(), Value::Byte(1_i8));
    data.insert(
        "dimension".to_string(),
        Value::String("minecraft:overworld".to_string()),
    );
    data.insert("trackingPosition".to_string(), Value::Byte(0_i8));
    data.insert("locked".to_string(), Value::Byte(1_i8));
    data.insert("unlimitedTracking".to_string(), Value::Byte(0_i8));
    data.insert("xCenter".to_string(), Value::Int(100000_i32));
    data.insert("ZCenter".to_string(), Value::Int(100000_i32));

    // Two empty lists for banner and frames (markers) in the NBT file
    data.insert("banners".to_string(), Value::List(Vec::new()));
    data.insert("frames".to_string(), Value::List(Vec::new()));

    // Two i64s to store the UUID (which in this case is unique but not random)
    data.insert("UUIDMost".to_string(), Value::Long(0_i64));
    data.insert("UUIDLeast".to_string(), Value::Long(index as i64));

    // Add the slice of pixels to the NBT file
    data.insert("colors".to_string(), Value::from(colors));

    // Used for the root unnamed tag
    let mut nbtfile = Blob::new();
    nbtfile.insert("data", Value::Compound(data))?;
    nbtfile.insert("DataVersion", Value::Int(2586_i32))?;

    let mut file = File::create(filename).unwrap();
    nbtfile.to_gzip_writer(&mut file)
}

fn generate_datapack(
    starting_index: i32,
    frames: i32,
    map_width: i32,
    map_height: i32,
    location: [i64; 3],
    facing: i8,
) -> Result<(), Box<dyn std::error::Error>> {
    let maps_per_frame = map_width * map_height;

    {
        let mut init =
            File::create("./out/datapacks/mapmaker/data/mapmaker/functions/init.mcfunction")
                .unwrap();
        write!(
            &mut init,
            include_str!("init_commands.in"),
            maps_per_frame,
            frames,
            frames * maps_per_frame,
            starting_index,
            location[0],
            location[1],
            location[2],
            facing,
            starting_index,
            starting_index,
            starting_index,
            starting_index
        )?;

        match facing {
            3 => {
                // South
                for i in starting_index + 1..starting_index + maps_per_frame {
                    let x = (i - starting_index) % map_width;
                    let z = 0;
                    write!(
                        &mut init,
                        include_str!("init_summon.in"),
                        starting_index,
                        x,
                        0 - ((i - starting_index) / map_width),
                        z,
                        facing,
                        i,
                        i,
                        i,
                        i
                    )?;
                }
            }
            4 => {
                // West
                for i in starting_index + 1..starting_index + maps_per_frame {
                    let x = 0;
                    let z = (i - starting_index) % map_width;
                    write!(
                        &mut init,
                        include_str!("init_summon.in"),
                        starting_index,
                        x,
                        0 - ((i - starting_index) / map_width),
                        z,
                        facing,
                        i,
                        i,
                        i,
                        i
                    )?;
                }
            }
            5 => {
                // East
                for i in starting_index + 1..starting_index + maps_per_frame {
                    let x = 0;
                    let z = 0 - ((i - starting_index) % map_width);
                    write!(
                        &mut init,
                        include_str!("init_summon.in"),
                        starting_index,
                        x,
                        0 - ((i - starting_index) / map_width),
                        z,
                        facing,
                        i,
                        i,
                        i,
                        i
                    )?;
                }
            }
            _ => {
                // 2 is default (North)
                for i in starting_index + 1..starting_index + maps_per_frame {
                    let x = 0 - ((i - starting_index) % map_width);
                    let z = 0;
                    write!(
                        &mut init,
                        include_str!("init_summon.in"),
                        starting_index,
                        x,
                        0 - ((i - starting_index) / map_width),
                        z,
                        facing,
                        i,
                        i,
                        i,
                        i
                    )?;
                }
            }
        }
    }
    {
        let mut loop_ =
            File::create("out/datapacks/mapmaker/data/mapmaker/functions/loop.mcfunction").unwrap();
        write!(&mut loop_, include_str!("loop_commands.in"))?;
        for i in starting_index..starting_index + maps_per_frame {
            write!(&mut loop_, include_str!("loop_scoreboard.in"), i, i, i, i)?;
        }
    }
    {
        let mut restart =
            File::create("out/datapacks/mapmaker/data/mapmaker/functions/restart.mcfunction")
                .unwrap();
        for i in starting_index..starting_index + maps_per_frame {
            writeln!(
                &mut restart,
                "scoreboard players set @e[tag={}] map_num {}",
                i, i
            )?;
        }
    }
    {
        // Write ID Counts file to prevent new maps from overwriting the generated ones
        let mut idcounts = File::create("out/data/idcounts.dat").unwrap();
        let mut idcounts_data: Map<String, Value> = Map::new();
        idcounts_data.insert(
            "map".to_string(),
            Value::Int((starting_index + frames * maps_per_frame) as i32),
        );
        let mut idcounts_file = Blob::new();
        idcounts_file.insert("data", Value::Compound(idcounts_data))?;
        idcounts_file.insert("DataVersion", Value::Int(2586_i32))?;
        idcounts_file.to_gzip_writer(&mut idcounts)?;
    }

    Ok(())
}
