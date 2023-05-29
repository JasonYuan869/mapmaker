

/// Direction enum with values that correspond to Minecraft's NBT `Facing` byte
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Direction {
    NORTH = 2,
    SOUTH = 3,
    WEST = 4,
    EAST = 5,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "north" => Direction::NORTH,
            "south" => Direction::SOUTH,
            "west" => Direction::WEST,
            "east" => Direction::EAST,
            _ => Direction::EAST,
        }
    }
}

/// A 3-tuple representing (x, y, z) coordinates in Minecraft
pub type Location = (i64, i64, i64);