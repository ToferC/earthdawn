use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use earthdawn::{systems::{setup, BulletPlugin, PlayerPlugin}, resources::Money};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Earthdawn".into(),
                    resolution: (1280.0, 960.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .run();
}
