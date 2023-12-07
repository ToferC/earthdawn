use bevy::{prelude::*, render::camera::ScalingMode};
use crate::components::{Player, Enemy};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 400.0,
        min_height: 200.0,
    };

    commands.spawn(camera);

    let hero_texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        texture: hero_texture,
        ..default()
    })
    .insert(Player::default());

    let ghoul_texture = asset_server.load("ghoul.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        texture: ghoul_texture,
        ..default()
    })
    .insert(Enemy::default());
}