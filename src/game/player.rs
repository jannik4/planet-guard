use super::{ApplyVelocity, SpaceShip, Steering, UpdateSpaceShip};
use crate::AppState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn cleanup(mut commands: Commands) {}