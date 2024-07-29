use super::{
    ApplyVelocity, Health, Level, SpaceShip, SpaceShipBundle, Steering, UpdateSpaceShip, Velocity,
};
use crate::{assets::GameAssets, AppState};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            update
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
    pub fn new(position: Vec3, rotation: f32, level: &Level, assets: &GameAssets) -> Self {
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
                assets,
            ),
        }
    }
}

fn update(
    mut space_ships: Query<&mut SpaceShip, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level: Res<Level>,
) {
    let Ok(mut space_ship) = space_ships.get_single_mut() else {
        return;
    };

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
