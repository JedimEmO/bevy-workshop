mod examples;

use bevy::prelude::*;
use clap::{Parser, Subcommand};
use crate::examples::projection::ProjectionExamplePlugin;
use crate::examples::simple_cube::SimpleCubeExamplePlugin;

#[derive(Subcommand, Default)]
enum Example {
    #[default]
    Projection,
    SimpleCube
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
        Example::Projection => {
            app.add_plugins(ProjectionExamplePlugin);
        }
        Example::SimpleCube => {
            app.add_plugins(SimpleCubeExamplePlugin);
        }
    }

    app.run();
}
