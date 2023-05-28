// These constants are copied directly
pub(super) const LOOP_CHECK_MCFUNCTION: &str = include_str!("mapmaker/functions/loop_check.mcfunction");
pub(super) const RENDER_MCFUNCTION: &str = include_str!("mapmaker/functions/render.mcfunction");
pub(super) const PACK_MCMETA: &str = include_str!("pack.mcmeta");
pub(super) const LOAD_JSON: &str = include_str!("minecraft/tags/functions/load.json");
pub(super) const TICK_JSON: &str = include_str!("minecraft/tags/functions/tick.json");

// These constants are templates to be prefilled
pub(super) const LOOP_COMMANDS_IN: &str = include_str!("mapmaker/functions/templates/loop_commands.in");
pub(super) const LOOP_SCOREBOARD_IN: &str = include_str!("mapmaker/functions/templates/loop_scoreboard.in");
pub(super) const INIT_COMMANDS_IN: &str = include_str!("mapmaker/functions/templates/init_commands.in");
pub(super) const INIT_SUMMON_IN: &str = include_str!("mapmaker/functions/templates/init_summon.in");