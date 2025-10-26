use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct DefaultExamplePlugin;
impl Plugin for DefaultExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_default_example);
        app.add_systems(Update, orbit_system);
    }
}

#[derive(Component)]
struct Orbit {
    radius: f32,
    speed: f32,
}

pub fn setup_default_example(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    ambient_light.brightness = 15.0;

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 8.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Orbiting light
    let mut light_direction_gizmo = GizmoAsset::new();

    light_direction_gizmo.arrow(Vec3::ZERO, -Vec3::Z, tailwind::YELLOW_400);

    commands.spawn((
        SpotLight {
            color: tailwind::YELLOW_400.into(),
            intensity: 500000.0,
            range: 20.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(-5.0, 2.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        Orbit {
            radius: 3.0,
            speed: 0.2,
        },
        Gizmo {
            handle: gizmo_assets.add(light_direction_gizmo),
            line_config: GizmoLineConfig {
                width: 1.,
                ..default()
            },
            ..default()
        }
    ));
}

fn orbit_system(time: Res<Time>, mut orbit_query: Query<(&mut Transform, &Orbit)>) {
    let time_total = time.elapsed_secs();

    for (mut transform, orbit) in orbit_query.iter_mut() {
        let angle = time_total * PI * 2. * orbit.speed;
        transform.translation.x = angle.cos() * orbit.radius;
        transform.translation.z = angle.sin() * orbit.radius;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
