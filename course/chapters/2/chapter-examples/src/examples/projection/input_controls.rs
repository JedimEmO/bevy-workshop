use std::f32::consts::PI;
use bevy::camera;
use bevy::prelude::*;

pub fn input_controls_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<Entity, With<Camera>>,
) {
    if keys.any_just_pressed([KeyCode::Digit1, KeyCode::Digit2]) {
        for camera_entity in cameras.iter() {
           commands.entity(camera_entity).despawn();
        }
    }

    if keys.just_pressed(KeyCode::Digit1) {
        spawn_default_3d_camera(&mut commands);
    } else if keys.just_pressed(KeyCode::Digit2) {
        spawn_orhtographic_3d_camera(&mut commands);
    }
}

pub fn spawn_default_3d_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.5, 1.0).looking_at(Vec3::new(0., 0., -1.), Vec3::Y),
        Projection::from(PerspectiveProjection {
            fov: PI/4.0,
            ..default()
        })
    ));
}

fn spawn_orhtographic_3d_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0.5, 1.0).looking_at(Vec3::new(0., 0., -1.), Vec3::Y),
        Projection::from(OrthographicProjection {
            scaling_mode: camera::ScalingMode::FixedVertical {
                viewport_height: 1.0,
            },
            ..OrthographicProjection::default_3d()
        }),
    ));
}
