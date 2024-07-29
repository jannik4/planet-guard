use super::{
    ApplyVelocity, Collider, GameState, Health, Level, Planet, SpaceShip, SpaceShipBundle,
    SpawnExplosion, Star, Steering, UpdateSpaceShip, Velocity,
};
use crate::{
    assets::{AudioAssets, GameAssets},
    AppState,
};
use bevy::prelude::*;
use rand::Rng;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            (update, dead)
                .before(UpdateSpaceShip)
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub space_ship: SpaceShipBundle,
}

impl PlayerBundle {
    pub fn new(
        position: Vec3,
        rotation: f32,
        level: &Level,
        audio_assets: &AudioAssets,
        assets: &GameAssets,
    ) -> Self {
        Self {
            player: Player,
            health: level.player_health,
            space_ship: SpaceShipBundle::new(
                0b1,
                Velocity(Vec3::ZERO),
                position,
                rotation,
                assets.player_space_ship_material.clone(),
                assets.player_bullet_material.clone(),
                audio_assets,
                assets,
            ),
        }
    }
}

fn update(
    mut players: Query<(&mut SpaceShip, &mut Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    level: Res<Level>,
) {
    let Ok((mut space_ship, mut transform)) = players.get_single_mut() else {
        return;
    };

    transform.scale = Vec3::lerp(
        transform.scale,
        Vec3::ONE,
        1.0 - f32::exp(f32::ln(0.95) * 60.0 * time.delta_seconds()),
    );

    match **game_state {
        GameState::Running | GameState::GameWon => {
            space_ship.steering = match (
                keyboard_input.pressed(KeyCode::KeyA),
                keyboard_input.pressed(KeyCode::KeyD),
            ) {
                (true, false) => Steering::Left,
                (false, true) => Steering::Right,
                _ => Steering::None,
            };
            space_ship.throttle = keyboard_input.pressed(KeyCode::KeyW);
            space_ship.brake = keyboard_input.pressed(KeyCode::KeyS);
            space_ship.shoot = keyboard_input
                .just_pressed(KeyCode::Space)
                .then_some(level.player_damage);
        }
        GameState::GameOver => {
            space_ship.steering = Steering::None;
            space_ship.throttle = false;
            space_ship.brake = true;
            space_ship.shoot = None;
        }
    }
}

fn dead(
    mut explosions: EventWriter<SpawnExplosion>,
    mut players: Query<
        (
            &mut Transform,
            &mut Velocity,
            &Collider,
            &mut Health,
            &SpaceShip,
        ),
        With<Player>,
    >,
    planets_and_stars: Query<
        (&Transform, &Collider),
        (Without<Player>, Or<(With<Planet>, With<Star>)>),
    >,
    level: Res<Level>,
) {
    for (mut transform, mut velocity, collider, mut health, space_ship) in &mut players {
        let mut dead = health.current() <= 0.0;

        if !dead {
            for (obj_transform, obj_collider) in &planets_and_stars {
                if Vec3::distance_squared(transform.translation, obj_transform.translation)
                    <= f32::powi(collider.radius + obj_collider.radius, 2)
                {
                    dead = true;
                    break;
                }
            }
        }

        if dead {
            explosions.send(SpawnExplosion {
                position: transform.translation,
                material: space_ship.material(),
            });

            let respawn_pos = 'respawn: loop {
                let respawn_pos = Vec3::new(
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    rand::thread_rng().gen_range(-500.0..=500.0),
                    0.0,
                );
                if respawn_pos.length() > 500.0 {
                    continue;
                }

                for (obj_transform, _obj_collider) in &planets_and_stars {
                    if Vec3::distance_squared(respawn_pos, obj_transform.translation) <= 70.0 {
                        continue 'respawn;
                    }
                }

                break respawn_pos;
            };

            transform.translation = respawn_pos;
            transform.scale = Vec3::splat(5.0);
            **velocity = Vec3::ZERO;
            *health = level.player_health;
        }
    }
}
