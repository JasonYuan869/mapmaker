use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use nbt::{Blob, Map, Value};

use crate::cli::constants::{Direction, Location};
use crate::image_processor::colors::MapColor;
use crate::output_generator::datapacks::{LOAD_JSON, LOOP_CHECK_MCFUNCTION, PACK_MCMETA, RENDER_MCFUNCTION, TICK_JSON};

mod datapacks;

/// Used in the `DataVersion` field of the NBT file.
/// Currently set to correspond to 1.16.5
const NBT_DATAVERSION: i32 = 2586;

/// This struct is responsible for outputting the NBT and datapack files.
/// The lifetime is bound to the path reference.
pub struct Generator<'a> {
    path: &'a Path,
    starting_index: usize,
    top_left: Location,
    direction: Direction,
}

pub struct InitializedGenerator<'a> {
    generator: Generator<'a>,
    frames: usize,
    map_columns: usize,
    map_rows: usize,
    maps_per_frame: usize,
}

impl Generator<'_> {
    /// Creates a new generator to output the NBT and datapack files.
    /// It will empty the given directory if it exists and create the output directory structure
    ///
    /// Run `init()` first before generating any files.
    pub fn new(path: &Path, starting_index: usize, top_left: Location, direction: Direction) -> anyhow::Result<Generator> {
        if path.exists() {
            if !path.is_dir() {
                anyhow::bail!("output path is not a directory")
            }
            if path.canonicalize()? == Path::new(".").canonicalize().unwrap() {
                anyhow::bail!("output path is the current directory")
            }
            println!("Output directory already exists. Deleting...");
            fs::remove_dir_all(path)?;
        }
        println!("Creating output directory structure...");
        fs::create_dir_all(path.join("data"))?;
        fs::create_dir_all(path.join("datapacks/mapmaker/data/mapmaker/functions"))?;
        fs::create_dir_all(path.join("datapacks/mapmaker/data/minecraft/tags/functions"))?;
        Ok(
            Generator {
                path,
                starting_index,
                top_left,
                direction,
            }
        )
    }
}

impl<'a> Generator<'a> {
    /// Initialize the files needed for the datapack.
    /// Consumes the generator and returns an initialized generator.
    pub fn init_files(self, frames: usize, map_columns: usize, map_rows: usize) -> anyhow::Result<InitializedGenerator<'a>> {
        // Write the pack.mcmeta file
        {
            let mut pack_mcmeta = File::create(self.path.join("datapacks/mapmaker/pack.mcmeta"))?;
            pack_mcmeta.write_all(PACK_MCMETA.as_bytes())?;
        }

        // Write the loop_check.mcfunction file
        {
            let mut loop_mcfunction = File::create(self.path.join("datapacks/mapmaker/data/mapmaker/functions/loop_check.mcfunction"))?;
            loop_mcfunction.write_all(LOOP_CHECK_MCFUNCTION.as_bytes())?;
        }

        // Write the render.mcfunction file
        {
            let mut render_mcfunction = File::create(self.path.join("datapacks/mapmaker/data/mapmaker/functions/render.mcfunction"))?;
            render_mcfunction.write_all(RENDER_MCFUNCTION.as_bytes())?;
        }

        // Write the Minecraft init and tick files
        {
            let mut load_json = File::create(self.path.join("datapacks/mapmaker/data/minecraft/tags/functions/load.json"))?;
            load_json.write_all(LOAD_JSON.as_bytes())?;
        }
        {
            let mut tick_json = File::create(self.path.join("datapacks/mapmaker/data/minecraft/tags/functions/tick.json"))?;
            tick_json.write_all(TICK_JSON.as_bytes())?;
        }

        Ok(
            InitializedGenerator {
                generator: self,
                frames,
                map_columns,
                map_rows,
                maps_per_frame: map_columns * map_rows,
            }
        )
    }
}

impl InitializedGenerator<'_> {
    /// Generates the `.dat` file for a given image, which stores the
    /// map's color data in a Minecraft-readable format.
    pub fn generate_dat(&self, colors: &[MapColor], index: usize, frame: usize) -> anyhow::Result<()> {
        if self.frames == 0 || self.maps_per_frame == 0 {
            anyhow::bail!("uninitialized generator");
        }
        let map_index = frame * self.maps_per_frame + index;

        let filename = self.generator.path.join(format!("data/map_{map_index}.dat"));

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
        data.insert("UUIDLeast".to_string(), Value::Long(map_index as i64));

        // Add the slice of pixels to the NBT file
        data.insert("colors".to_string(), Value::from(colors));

        // Used for the root unnamed tag
        let mut nbtfile = Blob::new();
        nbtfile.insert("data", Value::Compound(data))?;
        nbtfile.insert("DataVersion", Value::Int(NBT_DATAVERSION))?;

        let mut file = File::create(filename)?;
        Ok(nbtfile.to_gzip_writer(&mut file)?)
    }

    /// Generates the `idcounts.dat` file to prevent newly opened maps in Minecraft
    /// from overwriting these generated ones.
    pub fn generate_idcounts(&self) -> anyhow::Result<()> {
        if self.frames == 0 || self.maps_per_frame == 0 {
            anyhow::bail!("uninitialized generator");
        }

        // Write ID Counts file to prevent new maps from overwriting the generated ones
        let mut idcounts = File::create(self.generator.path.join("data/idcounts.dat"))?;
        let mut idcounts_data: Map<String, Value> = Map::new();
        let last_map = (self.generator.starting_index + self.frames * self.maps_per_frame) as i32;
        idcounts_data.insert(
            "map".to_string(),
            Value::Int(last_map),
        );
        let mut idcounts_file = Blob::new();
        idcounts_file.insert("data", Value::Compound(idcounts_data))?;

        idcounts_file.insert("DataVersion", Value::Int(NBT_DATAVERSION))?;
        Ok(idcounts_file.to_gzip_writer(&mut idcounts)?)
    }

    pub fn generate_datapack(&self) -> anyhow::Result<()> {
        if self.frames == 0 || self.maps_per_frame == 0 {
            anyhow::bail!("uninitialized generator");
        }

        self.generate_init_mcfunction()?;
        self.generate_loop_mcfunction()?;
        self.generate_restart_mcfunction()?;

        Ok(())
    }

    fn generate_init_mcfunction(&self) -> anyhow::Result<()> {
        let mut init_mcfunction = File::create(self.generator.path.join("datapacks/mapmaker/data/mapmaker/functions/init.mcfunction"))?;
        // Write the init commands, this initializes the scoreboard
        // and summons the top left map
        write!(
            &mut init_mcfunction,
            include_str!("datapacks/mapmaker/functions/templates/init_commands.in"),
            maps_per_frame=self.maps_per_frame,
            frames=self.frames,
            total_maps=self.maps_per_frame * self.frames,
            starting_index=self.generator.starting_index,
            x=self.generator.top_left.0,
            y=self.generator.top_left.1,
            z=self.generator.top_left.2,
            direction=self.generator.direction as u8,
        )?;

        // Summon the remaining maps
        for i in 1..self.maps_per_frame {
            let x: i32;
            let y: i32;
            let z: i32;
            match self.generator.direction {
                Direction::North => {
                    x = 0 - (i % self.map_columns) as i32;
                    y = 0 - (i / self.map_columns) as i32;
                    z = 0;
                }
                Direction::South => {
                    x = (i % self.map_columns) as i32;
                    y = 0 - (i / self.map_columns) as i32;
                    z = 0;
                }
                Direction::West => {
                    x = 0;
                    y = 0 - (i / self.map_columns) as i32;
                    z = 0 - (i % self.map_columns) as i32;
                }
                Direction::East => {
                    x = 0;
                    y = 0 - (i / self.map_columns) as i32;
                    z = (i % self.map_columns) as i32;
                }
            }
            write!(
                &mut init_mcfunction,
                include_str!("datapacks/mapmaker/functions/templates/init_summon.in"),
                starting_index=self.generator.starting_index,
                x=x,
                y=y,
                z=z,
                direction=self.generator.direction as u8,
                i=i+self.generator.starting_index,
            )?;
        }
        Ok(())
    }

    fn generate_loop_mcfunction(&self) -> anyhow::Result<()> {
        let mut loop_mcfunction = File::create(self.generator.path.join("datapacks/mapmaker/data/mapmaker/functions/loop.mcfunction"))?;
        write!(
            &mut loop_mcfunction,
            include_str!("datapacks/mapmaker/functions/templates/loop_commands.in")
        )?;

        for i in 0..self.maps_per_frame {
            write!(
                &mut loop_mcfunction,
                include_str!("datapacks/mapmaker/functions/templates/loop_scoreboard.in"),
                i=i+self.generator.starting_index
            )?;
        }
        Ok(())
    }

    fn generate_restart_mcfunction(&self) -> anyhow::Result<()> {
        let mut restart_mcfunction = File::create(self.generator.path.join("datapacks/mapmaker/data/mapmaker/functions/restart.mcfunction"))?;

        for i in 0..self.maps_per_frame {
            write!(
                &mut restart_mcfunction,
                include_str!("datapacks/mapmaker/functions/templates/restart.in"),
                i=i+self.generator.starting_index
            )?;
        }
        Ok(())
    }
}
