use std::path::PathBuf;

use inquire::CustomType;

use constants::{Direction, Location};

pub mod constants;


pub struct CliArgs {
    pub top_left: Location,
    pub direction: Direction,
    pub starting_index: usize,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

pub fn run() -> anyhow::Result<CliArgs> {
    let x = CustomType::<i64>::new("Enter the x-coordinate of the top left corner")
        .with_error_message("Please enter a valid integer")
        .with_default(0)
        .prompt()?;

    let y = CustomType::<i64>::new("Enter the y-coordinate of the top left corner")
        .with_error_message("Please enter a valid integer")
        .with_default(0)
        .prompt()?;

    let z = CustomType::<i64>::new("Enter the z-coordinate of the top left corner")
        .with_error_message("Please enter a valid integer")
        .with_default(0)
        .prompt()?;

    let direction_opts = vec!["north", "east", "south", "west", "up", "down"];
    let direction = inquire::Select::new("Select the direction the maps will facing", direction_opts)
        .with_starting_cursor(0)
        .with_help_message("This can be found in the F3 menu by looking towards a direction")
        .prompt()?;

    let starting_index = CustomType::<usize>::new("Enter the starting index of the maps")
        .with_error_message("Please enter a valid integer")
        .with_help_message("If you already have maps in the world, this should be the index of the next map")
        .with_default(0)
        .prompt()?;

    let input_path = inquire::Text::new("Enter the path to the input folder")
        .with_help_message("This should be a folder containing the images you want to convert")
        .with_default("in/")
        .prompt()?;

    let output_path = inquire::Text::new("Enter the path to the output folder")
        .with_help_message("This folder will be overwritten if it already exists")
        .with_default("out/")
        .prompt()?;

    let confirmation = inquire::Confirm::new("All contents of the output folder will be overwritten. Continue?")
        .with_default(true)
        .prompt()?;

    if !confirmation {
        anyhow::bail!("user cancelled");
    }

    Ok(
        CliArgs {
            top_left: (x, y, z),
            direction: direction.into(),
            starting_index,
            input_path: PathBuf::from(input_path),
            output_path: PathBuf::from(output_path),
        }
    )
}