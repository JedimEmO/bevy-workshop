use crate::examples::setup_default_example::DefaultExamplePlugin;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct SimpleCubeExamplePlugin;

impl Plugin for SimpleCubeExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultExamplePlugin);
        app.add_systems(Startup, setup_simple_cube);
    }
}

fn setup_simple_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube = Cuboid::from_size(Vec3::splat(1.));
    let cube_mesh_handle: Handle<Mesh> = meshes.add(cube);

    let material_red = StandardMaterial::from_color(tailwind::RED_500);
    let material_handle: Handle<StandardMaterial> = materials.add(material_red);

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    ));
}
