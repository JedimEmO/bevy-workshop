use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::input_processing::WithAxisProcessingPipelineExt;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{ActionState, GamepadControlAxis, InputMap};
use std::f32::consts::PI;
use bevy::camera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct GameResources {
    bullet_mesh: Handle<Mesh>,
    bullet_material: Handle<StandardMaterial>,
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
enum GameActions {
    Fire,
    #[actionlike(Axis)]
    Rotation,
}

pub struct InputExample;

impl Plugin for InputExample {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GameActions>::default());
        app.add_systems(Startup, (example_setup, player_setup).chain());
        app.add_systems(Update, input_handling_system);
        app.add_systems(FixedUpdate, movement_system);
    }
}

fn input_handling_system(
    mut commands: Commands,
    game_resources: ResMut<GameResources>,
    query: Single<(&ActionState<GameActions>, &mut Transform), With<Player>>,
) {
    let (action, mut transform) = query.into_inner();

    if action.just_pressed(&GameActions::Fire) {
        // spawn the bullet at the tip of the gun
        let mut transform = *transform;

        let forward = -transform.forward().as_vec3().normalize();

        transform.translation += forward * 0.5;

        commands.spawn((
            Mesh3d(game_resources.bullet_mesh.clone()),
            MeshMaterial3d(game_resources.bullet_material.clone()),
            transform,
            Velocity(forward * 10.),
        ));
    }

    if let Some(axis_info) = action.axis_data(&GameActions::Rotation) {
        let rot = -PI * (1. + axis_info.value) / 2.;
        transform.rotation = Quat::from_axis_angle(Vec3::Y, rot - PI / 2.);
    }
}

fn movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity)>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();

        // Make sure we clean up entities far from the camera
        if transform.translation.length() > 20. {
            commands.entity(entity).despawn();
        }
    }
}

fn player_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube = Cuboid::from_size(Vec3::new(0.1, 0.5, 0.5));
    let cube_mesh_handle: Handle<Mesh> = meshes.add(cube);

    let material_red = StandardMaterial::from_color(tailwind::RED_300);
    let material_handle: Handle<StandardMaterial> = materials.add(material_red);

    let mut player = commands.spawn((
        Player,
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
    ));

    let input_map = InputMap::default()
        .with(GameActions::Fire, GamepadButton::South)
        .with_axis(
            GameActions::Rotation,
            GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.15),
        );

    player.insert(input_map);
}

fn example_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let bullet_mesh = meshes.add(Mesh::from(Sphere { radius: 0.1 }));
    let bullet_material = materials.add(StandardMaterial::from_color(tailwind::YELLOW_200));

    commands.insert_resource(GameResources {
        bullet_mesh,
        bullet_material,
    });

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: camera::ScalingMode::FixedVertical {
                viewport_height: 5.,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 5., 0.).looking_at(Vec3::ZERO, -Vec3::Z),
        PointLight {
            color: tailwind::YELLOW_100.into(),
            intensity: 100000.0,
            range: 10.0,
            radius: 0.1,
            ..default()
        }
    ));
}
