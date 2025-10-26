//! Exercise 2
//!
//! Build a house using primitives! The camera should orbit the house
//!

mod mesh;
mod orbit;

use crate::mesh::{GameResources, spawn_layer_geometry};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use orbit::Orbit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_layer_geometry).chain())
        .add_systems(Update, orbit::orbit_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)).look_at(Vec3::ZERO, Vec3::Y),
        Orbit {
            radius: 20.0,
            speed: 0.2,
        },
    ));

    // Materials
    let grass = materials.add(StandardMaterial::from_color(tailwind::GREEN_800));
    let brick = materials.add(StandardMaterial::from_color(tailwind::AMBER_900));
    let roof = materials.add(StandardMaterial::from_color(tailwind::GRAY_900));
    let window = materials.add(StandardMaterial {
        metallic: 1.0,
        clearcoat: 1.0,
        ..StandardMaterial::from_color(tailwind::BLUE_500)
    });

    // mesh
    let cube = meshes.add(Cuboid::from_size(Vec3::splat(1.)));

    commands.insert_resource(GameResources {
        cube,
        brick,
        window,
        roof,
    });

    // ground
    let plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.)));
    commands.spawn((
        Mesh3d(plane),
        MeshMaterial3d(grass.clone()),
        Transform::default(),
    ));

    // praise the sun
    commands.spawn((
        PointLight {
            color: tailwind::YELLOW_100.into(),
            intensity: 20000000.0,
            range: 40.0,
            radius: 0.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::splat(10.)),
    ));
}
