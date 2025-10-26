use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub fn spawn_example_geometry_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_blue = materials.add(StandardMaterial {
        metallic: 1.,
        ..StandardMaterial::from_color(tailwind::BLUE_500)
    });
    let material_red = materials.add(StandardMaterial::from_color(tailwind::RED_500));
    let material_green = materials.add(StandardMaterial::from_color(tailwind::GREEN_700));

    let sphere = meshes.add(Sphere::new(0.2));
    let cone = meshes.add(Cone::new(0.2, 0.4));
    let plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.)));

    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(material_red),
        Transform::from_translation(Vec3::new(0.3, 0.2, 0.0)),
    ));

    commands.spawn((
        Mesh3d(cone),
        MeshMaterial3d(material_blue),
        Transform::from_translation(Vec3::new(0.0, 0.2, -1.0)),
    ));

    commands.spawn((
        Mesh3d(plane),
        MeshMaterial3d(material_green),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        PointLight {
            color: tailwind::YELLOW_100.into(),
            intensity: 20000.0,
            range: 100.0,
            radius: 0.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.5, 1.0, 0.5),
    ));
}
