use std::f32::consts::PI;

use bevy::prelude::*;

use crate::output::Flicker;

pub fn flicker_system(
    mut flame: Query<&mut Transform, (With<Flicker>, With<Handle<Mesh>>)>,
    mut light: Query<(&mut PointLight, &mut Transform), (With<Flicker>, Without<Handle<Mesh>>)>,
    time: Res<Time>,
) {
    let s = time.elapsed_seconds();
    let a = (s * 6.0).cos() * 0.0125 + (s * 4.0).cos() * 0.025;
    let b = (s * 5.0).cos() * 0.0125 + (s * 3.0).cos() * 0.025;
    let c = (s * 7.0).cos() * 0.0125 + (s * 2.0).cos() * 0.025;
    let (mut light, mut light_transform) = light.single_mut();
    let mut flame_transform = flame.single_mut();
    light.intensity = 1600.0 + 3000.0 * (a + b + c);
    flame_transform.translation = Vec3::new(-1.0, 1.23, 0.0);
    flame_transform.look_at(Vec3::new(-1.0 - c, 1.7 - b, 0.0 - a), Vec3::X);
    flame_transform.rotate(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, PI / 2.0));
    light_transform.translation = Vec3::new(-1.0 - c, 1.7, 0.0 - a);
    flame_transform.translation = Vec3::new(-1.0 - c, 1.23, 0.0 - a);
}
