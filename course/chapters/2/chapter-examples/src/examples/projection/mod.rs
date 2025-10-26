use crate::examples::projection::input_controls::{input_controls_system, spawn_default_3d_camera};
use crate::examples::projection::spawn_geometry::spawn_example_geometry_system;
use bevy::prelude::*;

pub mod input_controls;
pub mod spawn_geometry;

pub struct ProjectionExamplePlugin;

impl Plugin for ProjectionExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                |mut commands: Commands| {
                    spawn_default_3d_camera(&mut commands);
                },
                spawn_example_geometry_system,
            ),
        )
        .add_systems(Update, input_controls_system);
    }
}
