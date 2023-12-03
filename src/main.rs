use bevy::prelude::*;
use earthdawn::systems::character_movement;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

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
