//! Exercise 1
//!
//! Make the ball bounce!
//!
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, implement_me_system)
        .run();
}

#[derive(Component)]
struct BouncyBall {
    x_velocity: f32,
}

// ANCHOR: solution
fn implement_me_system(time: Res<Time>, mut query: Single<(&mut BouncyBall, &mut Transform)>) {
    let (mut bouncy_ball, mut transform) = query.into_inner();

    transform.translation.x += bouncy_ball.x_velocity * time.delta_secs();

    if transform.translation.x.abs() > 1.0 {
        let overshoot = transform.translation.x.fract();

        bouncy_ball.x_velocity *= -1.0;

        transform.translation.x = transform.translation.x.signum() * (1.0 - overshoot);
    }
}
// ANCHOR_END: solution

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // setup camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        PointLight {
            color: Default::default(),
            intensity: 4000.0,
            range: 10.0,
            radius: 0.2,
            ..default()
        }
    ));

    // Spawn the ball
    let ball_material_handle = materials.add(StandardMaterial::from_color(tailwind::YELLOW_300));
    let ball_shape_handle = meshes.add(Sphere::new(0.2));

    commands.spawn((
        Mesh3d(ball_shape_handle),
        MeshMaterial3d(ball_material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
        BouncyBall { x_velocity: 0.5 },
    ));
}
