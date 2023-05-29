use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use image::imageops::overlay;
use image::RgbImage;
use crate::image_processor::colors::{BLACK_INDEX, MapColor, MINECRAFT_COLOR_TREE, MinecraftRgb};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub mod colors;

macro_rules! ceil_div {
    ($a:expr, $b:expr) => {
        $a / $b + if $a % $b != 0 { 1 } else { 0 }
    };
}

// For the Floyd-Steinberg dithering algorithm
const DITHERING_VECTORS: [[i32; 2]; 4] = [
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];
const DITHERING_FACTORS: [f32; 4] = [0.4375, 0.1875, 0.3125, 0.0625];

/// The image processor struct.
pub struct Processor {
    /// The width of the source image.
    width: u32,

    /// The height of the source image.
    height: u32,

    /// The number of maps needed horizontally to render the source image.
    /// Equal to `width / 128` rounded up.
    map_columns: u32,

    /// The width in pixels of all maps needed to render the source image.
    /// Equal to `map_columns * 128`.
    map_width: u32,

    /// The number of maps needed vertically to render the source image.
    /// Equal to `height / 128` rounded up.
    map_rows: u32,

    /// The height in pixels of all maps needed to render the source image.
    /// Equal to `map_rows * 128`.
    map_height: u32,
}

impl Processor {
    /// Creates a new `Processor` with uninitialized fields.
    /// The first processed file will initialize all fields.
    pub fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            map_columns: 0,
            map_width: 0,
            map_rows: 0,
            map_height: 0,
        }
    }

    /// Returns the number of maps per one image frame
    pub fn maps_per_frame(&self) -> usize {
        (self.map_rows * self.map_columns) as usize
    }

    // Initializes the fields based on the given width and height
    fn init(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.map_columns = ceil_div!(width, 128);
        self.map_width = self.map_columns * 128;
        self.map_rows = ceil_div!(height, 128);
        self.map_height = self.map_rows * 128;
    }

    /// Processes the given image file by resizing it to fit on a multiple of Minecraft maps.
    /// Errors if the image dimensions do not match the dimensions of the `Processor`.
    pub fn process_file(&mut self, source: &Path) -> anyhow::Result<RgbImage> {
        let image = image::open(source)?.to_rgb8();
        let (width, height) = image.dimensions();

        if self.width == 0 || self.height == 0 {
            self.init(width, height);
        } else if width != self.width || height != self.height {
            anyhow::bail!("image dimensions do not match");
        }

        // Create a new image with the dimensions that are a multiple of 128
        let mut map = RgbImage::new(self.map_width, self.map_height);

        // Overlay the source image onto the new image, centered
        overlay(
            &mut map,
            &image,
            (self.map_height - self.height) as i64 / 2,
            (self.map_width - self.width) as i64 / 2,
        );

        Ok(map)
    }

    /// Same as `convert_colors`, but parallelized by splitting the image into horizontal strips.
    #[cfg(feature = "parallel")]
    pub fn convert_colors_parallel(&self, image: RgbImage) -> Vec<MapColor> {
        let par = Arc::new(RwLock::new(image));
        let mut result = vec![BLACK_INDEX; (self.map_width * self.map_height) as usize];
    }

    /// Converts the colors in the given image to the closest Minecraft map color.
    /// Returns a vector of Minecraft map colors split by map
    pub fn convert_colors(&self, image: &mut RgbImage) -> Vec<[MapColor; 16384]> {
        let mut result = vec![[BLACK_INDEX; 16384]; (self.map_columns * self.map_rows) as usize];

        for y in 0..image.height() {
            for x in 0..image.width() {
                // map relates to the index of the map across all maps
                // map_px relates to the index of the pixel within a single map
                let map_x = x / 128;
                let map_px_x = x % 128;
                let map_y = y / 128;
                let map_px_y = y % 128;
                let map_idx = (map_y * self.map_columns + map_x) as usize;
                let map_px_idx = (map_px_y * 128 + map_px_x) as usize;

                let color = image.get_pixel(x, y);
                let (mc_idx, difference) = MINECRAFT_COLOR_TREE.find_closest(color);
                result[map_idx][map_px_idx] = mc_idx;

                // Continue if no error to propagate
                if difference == [0, 0, 0] {
                    continue;
                }

                let errors = difference.map(|err| err as f32 / 256.0);

                // Propagate errors to each of the four pixels according to Floyd-Steinberg
                for ([vx, vy], factor) in DITHERING_VECTORS.iter().zip(DITHERING_FACTORS) {
                    let x = x as i32 + *vx;
                    let y = y as i32 + *vy;

                    // Check bounds within image (y will never be negative)
                    if x < 0 || x as u32 >= image.width() || y as u32 >= image.height() {
                        continue;
                    }

                    let original_color = image.get_pixel_mut(x as u32, y as u32);
                    for (channel, error) in original_color.0.iter_mut().zip(errors) {
                        let value = *channel as f32 / 256.0 + error * factor;
                        if value >= 1.0 {
                            *channel = 255;
                        } else if value <= 0.0 {
                            *channel = 0;
                        } else {
                            *channel = (value * 256.0) as u8;
                        }
                    }
                }
            }
        }

        result
    }
}