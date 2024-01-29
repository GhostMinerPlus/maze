use std::f32::consts::PI;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    pbr::{NotShadowCaster, TransmittedShadowReceiver},
    prelude::*,
    render::view::ColorGrading,
};

#[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy_xpbd_3d::components::{AngularVelocity, Collider, LinearVelocity, RigidBody};

#[derive(Component)]
pub struct Flicker;

#[derive(Component)]
pub struct ExampleControls {
    pub diffuse_transmission: bool,
    pub specular_transmission: bool,
    pub color: bool,
}

pub struct ExampleState {
    pub diffuse_transmission: f32,
    pub specular_transmission: f32,
    pub thickness: f32,
    pub ior: f32,
    pub perceptual_roughness: f32,
    pub reflectance: f32,
    pub auto_camera: bool,
}

#[derive(Component)]
pub struct ExampleDisplay;

impl Default for ExampleState {
    fn default() -> Self {
        ExampleState {
            diffuse_transmission: 0.5,
            specular_transmission: 0.9,
            thickness: 1.8,
            ior: 1.5,
            perceptual_roughness: 0.12,
            reflectance: 0.5,
            auto_camera: true,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let icosphere_mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.9,
            subdivisions: 7,
        })
        .unwrap(),
    );

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.7 }));

    let plane_mesh = meshes.add(shape::Plane::from_size(2.0).into());

    let cylinder_mesh = meshes.add(
        Mesh::try_from(shape::Cylinder {
            radius: 0.5,
            height: 2.0,
            resolution: 50,
            segments: 1,
        })
        .unwrap(),
    );

    // Cube #1
    commands.spawn((
        RigidBody::Dynamic,
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(StandardMaterial { ..default() }),
            transform: Transform::from_xyz(0.25, 0.5, -2.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                1.4,
                3.7,
                21.3,
            )),
            ..default()
        },
        Collider::cuboid(0.7, 0.7, 0.7),
        ExampleControls {
            color: true,
            specular_transmission: false,
            diffuse_transmission: false,
        },
    ));

    // Cube #2
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh,
            material: materials.add(StandardMaterial { ..default() }),
            transform: Transform::from_xyz(-0.75, 0.7, -2.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.4,
                2.3,
                4.7,
            )),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: false,
            diffuse_transmission: false,
        },
    ));

    // Candle
    commands.spawn((
        PbrBundle {
            mesh: cylinder_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.9, 0.2, 0.3, 1.0),
                diffuse_transmission: 0.7,
                perceptual_roughness: 0.32,
                thickness: 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: false,
            diffuse_transmission: true,
        },
    ));

    // Candle Flame
    commands.spawn((
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                emissive: Color::ANTIQUE_WHITE * 20.0 + Color::ORANGE_RED * 4.0,
                diffuse_transmission: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 1.15, 0.0).with_scale(Vec3::new(0.1, 0.2, 0.1)),
            ..default()
        },
        Flicker,
        NotShadowCaster,
    ));

    // Glass Sphere
    commands.spawn((
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                specular_transmission: 0.9,
                diffuse_transmission: 1.0,
                thickness: 1.8,
                ior: 1.5,
                perceptual_roughness: 0.12,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: true,
            diffuse_transmission: false,
        },
    ));

    // R Sphere
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(0.9),
        AngularVelocity::ZERO,
        LinearVelocity::ZERO,
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                specular_transmission: 0.9,
                diffuse_transmission: 1.0,
                thickness: 1.8,
                ior: 1.5,
                perceptual_roughness: 0.12,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, -0.5, 2.0).with_scale(Vec3::splat(0.5)),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: true,
            diffuse_transmission: false,
        },
        ExampleDisplay {},
    ));

    // G Sphere
    commands.spawn((
        PbrBundle {
            mesh: icosphere_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                specular_transmission: 0.9,
                diffuse_transmission: 1.0,
                thickness: 1.8,
                ior: 1.5,
                perceptual_roughness: 0.12,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.5, 2.0).with_scale(Vec3::splat(0.5)),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: true,
            diffuse_transmission: false,
        },
    ));

    // B Sphere
    commands.spawn((
        PbrBundle {
            mesh: icosphere_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                specular_transmission: 0.9,
                diffuse_transmission: 1.0,
                thickness: 1.8,
                ior: 1.5,
                perceptual_roughness: 0.12,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, -0.5, 2.0).with_scale(Vec3::splat(0.5)),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: true,
            diffuse_transmission: false,
        },
    ));

    // Plane
    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        reflectance: 0.3,
        perceptual_roughness: 0.8,
        ..default()
    });

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(2.0, 0.002, 2.0),
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: white_material.clone(),
            transform: Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3 {
                x: 100.0,
                y: 1.0,
                z: 100.0,
            }),
            ..default()
        },
        ExampleControls {
            color: true,
            specular_transmission: false,
            diffuse_transmission: false,
        },
    ));

    // Paper
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 0.002, 2.0),
        PbrBundle {
            mesh: plane_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                diffuse_transmission: 0.6,
                perceptual_roughness: 0.8,
                reflectance: 1.0,
                double_sided: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, -3.0)
                .with_scale(Vec3::new(2.0, 1.0, 1.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
            ..default()
        },
        TransmittedShadowReceiver,
        ExampleControls {
            specular_transmission: false,
            color: false,
            diffuse_transmission: true,
        },
    ));

    // Candle Light
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(-1.0, 1.7, 0.0),
            point_light: PointLight {
                color: Color::ANTIQUE_WHITE * 0.8 + Color::ORANGE_RED * 0.2,
                intensity: 1600.0,
                radius: 0.2,
                range: 5.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        Flicker,
    ));

    // Sun Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        directional_light: DirectionalLight {
            color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(1.0, 1.8, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
            color_grading: ColorGrading {
                exposure: -2.0,
                post_saturation: 1.2,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
        TemporalAntiAliasBundle::default(),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
        BloomSettings::default(),
    ));
}
