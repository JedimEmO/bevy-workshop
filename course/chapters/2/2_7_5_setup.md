# The final setup

Now we have all the pieces we need to set up our game world!

Let's spawn in two cubes, a ground plane and a camera!

```rust

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>) {
    // Geometry
    let cube = Cuboid::from_size(Vec3::splat(1.));
    let cube_mesh_handle: Handle<Mesh> = meshes.add(cube);

    let ground_plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.)));

    let material_red = StandardMaterial::from_color(tailwind::RED_500);
    let material_ground = StandardMaterial::from_color(tailwind::GREEN_500);

    let material_handle_red: Handle<StandardMaterial> = materials.add(material_red);
    let material_handle_ground: Handle<StandardMaterial> = materials.add(material_ground);

    commands.spawn((
        Mesh3d(cube_mesh_handle.clone()),
        MeshMaterial3d(material_handle_red.clone()),
        Transform::from_translation(Vec3::new(1.0, 0.5, 0.0))
    ));

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(material_handle_red),
        Transform::from_translation(Vec3::new(-1.0, 0.5, 0.0))
    ));

    commands.spawn((
        Mesh3d(ground_plane),
        MeshMaterial3d(material_handle_ground),
        Transform::from_translation(Vec3::ZERO),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0.5, 4.).looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y)
    ));
}
```