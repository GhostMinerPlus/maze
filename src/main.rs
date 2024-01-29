// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::{pbr::PointLightShadowMap, prelude::*};
use bevy_xpbd_3d::prelude::*;

#[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod effect;
mod fps;
mod input;
mod output;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        FrameTimeDiagnosticsPlugin::default(),
    ))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(PointLightShadowMap { size: 2048 })
    .insert_resource(AmbientLight {
        brightness: 0.0,
        ..default()
    })
    .add_systems(Startup, (output::setup, fps::setup_fps_counter))
    .add_systems(
        Update,
        (
            input::deal_input,
            effect::flicker_system,
            fps::fps_text_update_system,
            fps::fps_counter_showhide,
        ),
    );

    // *Note:* TAA is not _required_ for specular transmission, but
    // it _greatly enhances_ the look of the resulting blur effects.
    // Sadly, it's not available under WebGL.
    #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
    app.insert_resource(Msaa::Off)
        .add_plugins(TemporalAntiAliasPlugin);

    app.run();
}
