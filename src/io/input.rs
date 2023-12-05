//! This example showcases light transmission
//!
//! ## Controls
//!
//! | Key Binding        | Action                                               |
//! |:-------------------|:-----------------------------------------------------|
//! | `J`/`K`/`L`/`;`    | Change Screen Space Transmission Quality             |
//! | `O` / `P`          | Decrease / Increase Screen Space Transmission Steps  |
//! | `1` / `2`          | Decrease / Increase Diffuse Transmission             |
//! | `Q` / `W`          | Decrease / Increase Specular Transmission            |
//! | `A` / `S`          | Decrease / Increase Thickness                        |
//! | `Z` / `X`          | Decrease / Increase IOR                              |
//! | `E` / `R`          | Decrease / Increase Perceptual Roughness             |
//! | `U` / `I`          | Decrease / Increase Reflectance                      |
//! | Arrow Keys         | Control Camera                                       |
//! | `C`                | Randomize Colors                                     |
//! | `H`                | Toggle HDR + Bloom                                   |
//! | `D`                | Toggle Depth Prepass                                 |
//! | `T`                | Toggle TAA                                           |
use bevy::{input::mouse::MouseMotion, prelude::*};

#[allow(clippy::too_many_arguments)]
pub fn deal_input(
    mut camera: Query<(&mut Transform,), With<Camera3d>>,
    input: Res<Input<KeyCode>>,
    mouse_events: Res<'_, Events<MouseMotion>>,
) {
    let (mut camera_transform,) = camera.single_mut();

    let mut c_x =
        camera_transform.transform_point(Vec3::X) - camera_transform.transform_point(Vec3::ZERO);
    c_x.y = 0.0;
    c_x = c_x.normalize();
    let c_z = c_x.cross(Vec3::Y);

    if input.pressed(KeyCode::W) {
        camera_transform.translation -= c_z;
    } else if input.pressed(KeyCode::S) {
        camera_transform.translation += c_z;
    }

    if input.pressed(KeyCode::A) {
        camera_transform.translation -= c_x;
    } else if input.pressed(KeyCode::D) {
        camera_transform.translation += c_x;
    }

    if input.pressed(KeyCode::Space) {
        camera_transform.translation += Vec3::Y * 0.1;
    } else if input.pressed(KeyCode::C) {
        camera_transform.translation -= Vec3::Y * 0.1;
    }

    let mut reader = mouse_events.get_reader();
    for e in reader.read(mouse_events.as_ref()) {
        let axis = -e.delta.y * c_x - e.delta.x * Vec3::Y;
        camera_transform.rotate_axis(axis, 0.001);

        c_x = camera_transform.transform_point(Vec3::X)
            - camera_transform.transform_point(Vec3::ZERO);
        c_x.y = 0.0;
        c_x = c_x.normalize();
    }
}
