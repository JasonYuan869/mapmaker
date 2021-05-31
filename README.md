# mapmaker
An experimental program written in Rust that converts image files to a Minecraft-compatible map format. When provided with a series of image files with the same dimensions, it outputs a datapack that allows the sequence to be animated at 20 frames per second (with some limitations).

854x480 Source | Minecraft Maps | Zoomed
---|---|---
<img src="https://user-images.githubusercontent.com/18143408/120138357-52a0cf80-c1a4-11eb-85df-f51ad46ab42e.jpg"> | <img src="https://user-images.githubusercontent.com/18143408/120136896-33ed0980-c1a1-11eb-8eb8-7eb7d8fa797d.png"> | <img src="https://user-images.githubusercontent.com/18143408/120136915-3e0f0800-c1a1-11eb-8086-1ebd77461b55.png">

## Features
- Support for any dimension of source image (the program will add maps to fit the dimensions as necessary)
- Support for PNG and JPEG files (although some PNG files seem to be incompatible for some reason)
- Dithering to reduce color banding and simulate a greater variety of colors
- Generates a datapack that automatically summons the item frames at a specific location and advances to the next frame for animations
  - The location is currently locked at (0 100 0) facing north, but the ability to change this is WIP
- Technically will work on servers, but every client must be near the map during the loading process to avoid flickering (see the last paragraph of the next section)

## Limitations
- The maps will only generate at a specific location (currently 0 100 0) facing north
- Long wait time to load the maps in Minecraft (see the last paragraph of the next section)
  - Takes 10x the duration of the source footage
  - Not an issue for single frame images
- Limited color scheme (204 visible colors only, down from ~16 million)
  - Somewhat compensated by dithering
- Very resource intensive
  - Causes server side lag (even in singleplayer) as the garbage collector attempts to save memory
  - Can crash Minecraft when many maps are loaded in, making it unusable for longer durations or very high quality sources
    - About 15,000 maps with 6 GB of RAM allocated to Minecraft (variable)

## Instructions
To use this program, first create a new superflat void world. This step is technically unnecessary but the datapack will always summon the item frames around (0 100 0).

If converting a video, use `ffmpeg` to first convert it into a series of images (read [ffmpeg manual](https://ffmpeg.org/ffmpeg.html) or ask Google).
In both cases below, it may be easier to use symlinks instead of copying files to/from the program folder, especially if the source is long or the quality is high.

### Using the provided binary (Windows)
1. Download the .zip file from the releases page and unzip.
2. Place the sources images in `in`.
3. Run `mapmaker.exe`.
4. Copy/move all files in `out` to the base of your world save.
5. Open the world in Minecraft.

### Building from source
Prerequisites: `rust`, `cargo` (follow instructions [here](https://www.rust-lang.org/learn/get-started))
1. Clone this repo.
2. Place the sources images in `in`.
3. Run `cargo run --release` to build the program in release mode (significantly faster).
4. Copy/move all files in `out` to the base of your world save.
5. Open the world in Minecraft.

## How it Works
### Map NBT structure
Minecraft stores map files in the world save directory in gzipped NBT files located at `data/map_<id>.dat`, one file for every unique map. A map with a specific ID will have an integer NBT tag called `map` containing the ID. These can be given to a player or summoned into item frames with the following commands:
```mcfunction
give @p minecraft:filled_map{map: <ID>}
summon minecraft:item_frame ~ ~ ~ {Facing: <Direction_Byte>, Item: {id: "minecraft:filled_map", tag: {map: <ID>}, Count: 1b}}
```
Every `map_<ID>.dat` file contains an array of bytes with length 16384 representing the color of each pixel on the 128-by-128 map. Index 0 represents the top left corner of the map, and increasing the index will increase the horizontal offset, then the vertical.

There are 59 base colors available in Minecraft 1.16, however each base color is actually associated with four separate map colors of varying lightness making a total of 208 colors (204 excluding the four transparant colors).
The 208 colors are each associated with a color ID, which is the byte that is entered into the map's NBT file. In this program, I hardcoded the list of colors from the [Minecraft Wiki](https://minecraft.fandom.com/wiki/Map_item_format#Full_color_tables) into an array of RGB values found in `color_list.rs`. The `Rgb<u8>` struct comes from the [image crate](https://docs.rs/crate/image).

### Color conversion
Also in `color_list.rs`, there is a struct called `RgbColorMap` which takes an array of `Rgb<u8>` as its `colors` field to represent the array of availbe colors.
One function is implemented for this struct called `map_indices(&self, color: &Rgb<u8>)`. It takes a single parameter representing an RGB color, which is then processed to find the closest equivalent color in `colors`. It does this by iterating through `colors` and computing the sum of squared differences (Euclidean distace formula) of each RGB channel, then returning the index of the color with the lowest distance score. An array representing the error between the source color and the resulting color is also computed and returned in this function, to be used for dithering. The last thing in this file is a public static variable `MINECRAFT_COLOR_MAP`, which is an `RgbColorMap` with the array of Minecraft colors as its field.

When an input image is opened by the program, it is first resized to a multiple of 128 pixels on both dimensions so it fits perfectly on a whole number of maps. The borders of the resized image (if any) are colored black. The image crate stores images in memory as a container of pixels, similar to a vector, indexed starting from the top left going horizontally. The image is converted to a vector of color indices by iterating through every pixel and calling `map_indices(pixel_color)` of `MINECRAFT_COLOR_MAP`. The error of each pixel is propagated to neighboring pixels following the [Floyd-Steinberg dithering algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering), which is used to reduce color banding. This process is repeated for every input image in the folder.

Note that the final vector of indices is actually a 4D array with the following coordinate system: `(map_y, map_x, pixel_y, pixel_x)`.
The x- and y-coordinates of the map identifies a specific map in the grid of 128x128 maps (starting from the top left, going horizontally).
The x- and y-coordinates of the pixel is a series of 16384 bytes in the exact same format as in the map NBT data. This makes it very easy to generate each map file by passing a sequential slice of 16384 bytes from the vector.

Outputting NBT files is simply done using the [hematite-nbt crate](https://crates.io/crates/hematite-nbt). A .dat file is generated for every map by taking a slice of the vector of indices, which is then output in gzip format as `map_<id>.dat`. There is also an index parameter which ensure that new files are created for later frames, instead of overwriting existing files.

## The datapack
Since the datapack commands would depend on the number of frames and the dimensions of the frame, many of the datapack files are generated by the program itself. 

`init.mcfunction` contains the commands that are run when the world is loaded. This function initializes various scoreboard variables, including the number of frames and the number of maps per frame. It also summons item frames containing the maps for the first frame and with two special NBT tags representing the item frame's coordinates within the grid of maps and a miscellaneous tag called `mapmaker`.
```mcfunction
summon minecraft:item_frame 0 100 0 {Facing:2b, Fixed:1b, Item:{id:"minecraft:filled_map", tag:{map:0}, Count:1b}, Tags:["mapmaker", "0"]}
scoreboard players set @e[tag=0] map_num 0
summon minecraft:item_frame -1 100 0 {Facing:2b, Fixed:1b, Item:{id:"minecraft:filled_map", tag:{map:1}, Count:1b}, Tags:["mapmaker", "1"]}
scoreboard players set @e[tag=1] map_num 1
# Repeat for every item frame...
```

In `loop.mcfunction`, every item frame will have its map ID incremented by the number of maps per frame modulo total frames to show the next frame and loop from the start when finished. This is acheived using the `/data` command, which provides an interface between scoreboard variables and NBT data.
```mcfunction
scoreboard players operation @e[tag=mapmaker] map_num += Global maps_per_frame
scoreboard players operation @e[tag=mapmaker] map_num %= Global total_maps
execute as @e[tag=0] store result storage mapmaker:id id_0 int 1 run scoreboard players get @s map_num
execute as @e[tag=0] run data modify entity @s Item.tag.map set from storage mapmaker:id id_0
execute as @e[tag=1] store result storage mapmaker:id id_1 int 1 run scoreboard players get @s map_num
execute as @e[tag=1] run data modify entity @s Item.tag.map set from storage mapmaker:id id_1
# Repeat for every item frame...
```

There is a file called `loop_check.mcfunction`, which is what Minecraft runs every tick. It stops execution if the animation is paused with the following command:
```mcfunction
# Set to 1 to pause, 0 to unpause
scoreboard players set Global paused 1
```
It also prepares the animation first by loading one frame every 10 ticks (2 frames per second). Although slow, this step is necessary to prevent flickering of the screen. Minecraft takes approximately 10 ticks to load a map from its file into memory and render it to the screen. If the map is not loaded in completely, it will show as transparent which causes major flickering in the animation. Once all frames are properly loaded into memory, the animation can finally proceed at full speed.
