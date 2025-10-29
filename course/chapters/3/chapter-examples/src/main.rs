mod examples;

use bevy::prelude::*;
use clap::{Parser, Subcommand};
use crate::examples::input::InputExample;

#[derive(Subcommand, Default)]
enum Example {
    #[default]
    Input
}

#[derive(Parser)]
struct CliParams {
    #[clap(subcommand)]
    example: Example,
}

fn main() {
    let cli_params = CliParams::parse();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    match cli_params.example {
        Example::Input => {
            app.add_plugins(InputExample);
        }
    }

    app.run();
}
