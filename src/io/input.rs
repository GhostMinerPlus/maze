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
use bevy::{
    core_pipeline::{
        core_3d::ScreenSpaceTransmissionQuality, prepass::DepthPrepass,
    },
    prelude::*,
    render::camera::TemporalJitter,
};

#[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasSettings;

use rand::random;

use crate::io::output::{ExampleControls, ExampleDisplay, ExampleState};

#[allow(clippy::too_many_arguments)]
pub fn deal_input(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    controllable: Query<(&Handle<StandardMaterial>, &ExampleControls)>,
    mut camera: Query<
        (
            Entity,
            &mut Camera,
            &mut Camera3d,
            &mut Transform,
            Option<&DepthPrepass>,
            Option<&TemporalJitter>,
        ),
        With<Camera3d>,
    >,
    mut display: Query<&mut Text, With<ExampleDisplay>>,
    mut state: Local<ExampleState>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    // camera
    if input.pressed(KeyCode::Key2) {
        state.diffuse_transmission = (state.diffuse_transmission + time.delta_seconds()).min(1.0);
    } else if input.pressed(KeyCode::Key1) {
        state.diffuse_transmission = (state.diffuse_transmission - time.delta_seconds()).max(0.0);
    }

    if input.pressed(KeyCode::W) {
        state.specular_transmission = (state.specular_transmission + time.delta_seconds()).min(1.0);
    } else if input.pressed(KeyCode::Q) {
        state.specular_transmission = (state.specular_transmission - time.delta_seconds()).max(0.0);
    }

    if input.pressed(KeyCode::S) {
        state.thickness = (state.thickness + time.delta_seconds()).min(5.0);
    } else if input.pressed(KeyCode::A) {
        state.thickness = (state.thickness - time.delta_seconds()).max(0.0);
    }

    if input.pressed(KeyCode::X) {
        state.ior = (state.ior + time.delta_seconds()).min(3.0);
    } else if input.pressed(KeyCode::Z) {
        state.ior = (state.ior - time.delta_seconds()).max(1.0);
    }

    if input.pressed(KeyCode::I) {
        state.reflectance = (state.reflectance + time.delta_seconds()).min(1.0);
    } else if input.pressed(KeyCode::U) {
        state.reflectance = (state.reflectance - time.delta_seconds()).max(0.0);
    }

    if input.pressed(KeyCode::R) {
        state.perceptual_roughness = (state.perceptual_roughness + time.delta_seconds()).min(1.0);
    } else if input.pressed(KeyCode::E) {
        state.perceptual_roughness = (state.perceptual_roughness - time.delta_seconds()).max(0.0);
    }

    let randomize_colors = input.just_pressed(KeyCode::C);

    for (material_handle, controls) in &controllable {
        let material = materials.get_mut(material_handle).unwrap();
        if controls.specular_transmission {
            material.specular_transmission = state.specular_transmission;
            material.thickness = state.thickness;
            material.ior = state.ior;
            material.perceptual_roughness = state.perceptual_roughness;
            material.reflectance = state.reflectance;
        }

        if controls.diffuse_transmission {
            material.diffuse_transmission = state.diffuse_transmission;
        }

        if controls.color && randomize_colors {
            material.base_color.set_r(random());
            material.base_color.set_g(random());
            material.base_color.set_b(random());
        }
    }

    let (
        camera_entity,
        mut camera,
        mut camera_3d,
        mut camera_transform,
        depth_prepass,
        temporal_jitter,
    ) = camera.single_mut();

    if input.just_pressed(KeyCode::H) {
        camera.hdr = !camera.hdr;
    }

    #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
    if input.just_pressed(KeyCode::D) {
        if depth_prepass.is_none() {
            commands.entity(camera_entity).insert(DepthPrepass);
        } else {
            commands.entity(camera_entity).remove::<DepthPrepass>();
        }
    }

    #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
    if input.just_pressed(KeyCode::T) {
        if temporal_jitter.is_none() {
            commands.entity(camera_entity).insert((
                TemporalJitter::default(),
                TemporalAntiAliasSettings::default(),
            ));
        } else {
            commands
                .entity(camera_entity)
                .remove::<(TemporalJitter, TemporalAntiAliasSettings)>();
        }
    }

    if input.just_pressed(KeyCode::O) && camera_3d.screen_space_specular_transmission_steps > 0 {
        camera_3d.screen_space_specular_transmission_steps -= 1;
    }

    if input.just_pressed(KeyCode::P) && camera_3d.screen_space_specular_transmission_steps < 4 {
        camera_3d.screen_space_specular_transmission_steps += 1;
    }

    if input.just_pressed(KeyCode::J) {
        camera_3d.screen_space_specular_transmission_quality = ScreenSpaceTransmissionQuality::Low;
    }

    if input.just_pressed(KeyCode::K) {
        camera_3d.screen_space_specular_transmission_quality =
            ScreenSpaceTransmissionQuality::Medium;
    }

    if input.just_pressed(KeyCode::L) {
        camera_3d.screen_space_specular_transmission_quality = ScreenSpaceTransmissionQuality::High;
    }

    if input.just_pressed(KeyCode::Semicolon) {
        camera_3d.screen_space_specular_transmission_quality =
            ScreenSpaceTransmissionQuality::Ultra;
    }

    let rotation = if input.pressed(KeyCode::Right) {
        state.auto_camera = false;
        time.delta_seconds()
    } else if input.pressed(KeyCode::Left) {
        state.auto_camera = false;
        -time.delta_seconds()
    } else if state.auto_camera {
        time.delta_seconds() * 0.25
    } else {
        0.0
    };

    let distance_change =
        if input.pressed(KeyCode::Down) && camera_transform.translation.length() < 25.0 {
            time.delta_seconds()
        } else if input.pressed(KeyCode::Up) && camera_transform.translation.length() > 2.0 {
            -time.delta_seconds()
        } else {
            0.0
        };

    camera_transform.translation *= distance_change.exp();

    camera_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
    );

    let mut display = display.single_mut();
    display.sections[0].value = format!(
        concat!(
            " J / K / L / ;  Screen Space Specular Transmissive Quality: {:?}\n",
            "         O / P  Screen Space Specular Transmissive Steps: {}\n",
            "         1 / 2  Diffuse Transmission: {:.2}\n",
            "         Q / W  Specular Transmission: {:.2}\n",
            "         A / S  Thickness: {:.2}\n",
            "         Z / X  IOR: {:.2}\n",
            "         E / R  Perceptual Roughness: {:.2}\n",
            "         U / I  Reflectance: {:.2}\n",
            "    Arrow Keys  Control Camera\n",
            "             C  Randomize Colors\n",
            "             H  HDR + Bloom: {}\n",
            "             D  Depth Prepass: {}\n",
            "             T  TAA: {}\n",
        ),
        camera_3d.screen_space_specular_transmission_quality,
        camera_3d.screen_space_specular_transmission_steps,
        state.diffuse_transmission,
        state.specular_transmission,
        state.thickness,
        state.ior,
        state.perceptual_roughness,
        state.reflectance,
        if camera.hdr { "ON " } else { "OFF" },
        if cfg!(any(not(feature = "webgl2"), not(target_arch = "wasm32"))) {
            if depth_prepass.is_some() {
                "ON "
            } else {
                "OFF"
            }
        } else {
            "N/A (WebGL)"
        },
        if cfg!(any(not(feature = "webgl2"), not(target_arch = "wasm32"))) {
            if temporal_jitter.is_some() {
                if depth_prepass.is_some() {
                    "ON "
                } else {
                    "N/A (Needs Depth Prepass)"
                }
            } else {
                "OFF"
            }
        } else {
            "N/A (WebGL)"
        }
    );
}