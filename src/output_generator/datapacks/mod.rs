use std::fs::File;
use std::io::Write;
use crate::cli::constants::{Direction, Location};

// These constants are files that are generated directly
pub(super) const LOOP_CHECK_MCFUNCTION: &str = include_str!("mapmaker/functions/loop_check.mcfunction");
pub(super) const RENDER_MCFUNCTION: &str = include_str!("mapmaker/functions/render.mcfunction");
pub(super) const PACK_MCMETA: &str = include_str!("pack.mcmeta");
pub(super) const LOAD_JSON: &str = include_str!("minecraft/tags/functions/load.json");
pub(super) const TICK_JSON: &str = include_str!("minecraft/tags/functions/tick.json");