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
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity};

use super::output::ExampleDisplay;

#[allow(clippy::too_many_arguments)]
pub fn deal_input(
    mut camera: Query<(&mut Transform,), With<Camera3d>>,
    mut r_ball: Query<
        (&mut AngularVelocity, &mut LinearVelocity, &mut Transform),
        (With<ExampleDisplay>, Without<Camera3d>),
    >,
    mut windows: Query<&mut Window>,
    input: Res<Input<KeyCode>>,
    mouse_events: Res<'_, Events<MouseMotion>>,
) {
    let mut ball = r_ball.single_mut();
    let (mut camera_transform,) = camera.single_mut();

    let mut c_x =
        camera_transform.transform_point(Vec3::X) - camera_transform.transform_point(Vec3::ZERO);
    c_x.y = 0.0;
    c_x = c_x.normalize();
    let mut c_z = c_x.cross(Vec3::Y);
    camera_transform.translation = ball.2.transform_point(Vec3::ZERO);
    camera_transform.translation += c_z * 2.0 + Vec3::Y;

    let mut window = windows.single_mut();
    if input.just_pressed(KeyCode::Escape) {
        if window.cursor.visible {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        } else {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
    if !window.cursor.visible {
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
    c_z = c_x.cross(Vec3::Y);

    ball.0.0 = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        ball.0.0 -= c_x * 5.0;
    } else if input.pressed(KeyCode::S) {
        ball.0.0 += c_x * 5.0;
    }
    if input.pressed(KeyCode::A) {
        ball.0.0 += c_z * 5.0;
    } else if input.pressed(KeyCode::D) {
        ball.0.0 -= c_z * 5.0;
    }
    if input.just_pressed(KeyCode::Space) {
        ball.1.y += 5.0;
    }
}
