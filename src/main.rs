use bevy::prelude::*;
use earthdawn::systems::{character_movement, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Earthdawn".into(),
                    resolution: (640.0, 480.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}
