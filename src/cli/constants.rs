/// Direction enum with values that correspond to Minecraft's NBT `Facing` byte
#[repr(u8)]
pub enum Direction {
    NORTH = 2,
    EAST = 5,
    SOUTH = 3,
    WEST = 4,
}