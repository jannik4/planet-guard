use super::{
    ApplyVelocity, Health, SpaceShip, SpaceShipBundle, Steering, UpdateSpaceShip, Velocity,
};
use crate::AppState;
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
    pub fn new(
        position: Vec3,
        rotation: f32,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            player: Player,
            health: Health(10.0),
            space_ship: SpaceShipBundle::new(
                0b1,
                Velocity(Vec3::ZERO),
                position,
                rotation,
                Color::srgb(0.6, 0.6, 1.4),
                Color::srgb(0.0, 0.0, 2.0),
                meshes,
                materials,
            ),
        }
    }
}

fn update(
    mut space_ships: Query<&mut SpaceShip, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
    space_ship.shoot = keyboard_input.just_pressed(KeyCode::Space);
}
