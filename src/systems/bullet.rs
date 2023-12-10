use bevy::prelude::*;
use crate::components::{Player, Bullet};
use crate::resources::Money;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
        Update,
        (spawn_bullet,
                bullet_lifetime),
        )
        .register_type::<Bullet>();
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a bullet, remaining money: ${:?}", money.0);

        let texture = asset_server.load("bullet.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Bullet {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
            Name::new("Bullet"),
        ));
    }
}

pub fn bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
    mut money: ResMut<Money>,
) {
    for (bullet_entity, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());

        if bullet.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(bullet_entity).despawn();

            info!("Bullet explodes, you gain {:?}", money.0);
        }
    }
}