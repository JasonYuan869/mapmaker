

/// Direction enum with values that correspond to Minecraft's NBT `Facing` byte
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Direction {
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "north" => Direction::North,
            "south" => Direction::South,
            "west" => Direction::West,
            "east" => Direction::East,
            _ => Direction::North,
        }
    }
}

/// A 3-tuple representing (x, y, z) coordinates in Minecraft
pub type Location = (i64, i64, i64);