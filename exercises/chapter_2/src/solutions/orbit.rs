use std::f32::consts::PI;
use bevy::prelude::{Component, Query, Res, Time, Transform};
use bevy::math::Vec3;

#[derive(Component)]
pub struct Orbit {
    pub radius: f32,
    pub speed: f32,
}

pub fn orbit_system(time: Res<Time>, mut orbit_query: Query<(&mut Transform, &Orbit)>) {
    let time_total = time.elapsed_secs();

    for (mut transform, orbit) in orbit_query.iter_mut() {
        let angle = time_total * PI * 2. * orbit.speed;
        transform.translation.x = angle.cos() * orbit.radius;
        transform.translation.y = (time_total * 0.8).sin() * 5. + 5.;
        transform.translation.z = angle.sin() * orbit.radius;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}