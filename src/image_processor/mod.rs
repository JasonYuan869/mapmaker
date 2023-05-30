use std::path::Path;

use image::{GenericImageView, RgbImage};
use image::imageops::overlay;

use crate::image_processor::colors::{BLACK_INDEX, MapColor, MINECRAFT_COLOR_TREE};

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
    pub map_columns: u32,

    /// The width in pixels of all maps needed to render the source image.
    /// Equal to `map_columns * 128`.
    map_width: u32,

    /// The number of maps needed vertically to render the source image.
    /// Equal to `height / 128` rounded up.
    pub map_rows: u32,

    /// The height in pixels of all maps needed to render the source image.
    /// Equal to `map_rows * 128`.
    map_height: u32,
}

impl Processor {
    pub fn new(first_image: &Path) -> anyhow::Result<Self> {
        let image = image::open(first_image)?;
        let (width, height) = image.dimensions();
        Ok(
            Processor {
                width,
                height,
                map_columns: ceil_div!(width, 128),
                map_width: ceil_div!(width, 128) * 128,
                map_rows: ceil_div!(height, 128),
                map_height: ceil_div!(height, 128) * 128,
            }
        )
    }

    /// Processes the given image file by resizing it to fit on a multiple of Minecraft maps.
    /// Errors if the image dimensions do not match the dimensions of the `Processor`.
    pub fn process_file(&self, source: &Path) -> anyhow::Result<RgbImage> {
        let image = image::open(source)?.to_rgb8();
        let (width, height) = image.dimensions();

        if width != self.width || height != self.height {
            anyhow::bail!("image dimensions do not match");
        }

        // Create a new image with the dimensions that are a multiple of 128
        let mut map = RgbImage::new(self.map_width, self.map_height);

        // Overlay the source image onto the new image, centered
        overlay(
            &mut map,
            &image,
            (self.map_width - self.width) as i64 / 2,
            (self.map_height - self.height) as i64 / 2,
        );

        Ok(map)
    }

    /// Converts the colors in the given image to the closest Minecraft map color.
    /// Returns a vector of Minecraft map colors split by map
    pub fn convert_colors(&self, mut image: RgbImage) -> Vec<[MapColor; 16384]> {
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