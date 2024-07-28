use super::{SpaceShip, Velocity};
use crate::AppState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(Update, update.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Component)]
pub struct Player;

fn update(
    time: Res<Time>,
    mut space_ships: Query<(&mut SpaceShip, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut space_ship, mut velocity)) = space_ships.get_single_mut() else {
        return;
    };

    let rotation = keyboard_input.pressed(KeyCode::KeyA) as u8 as f32
        - keyboard_input.pressed(KeyCode::KeyD) as u8 as f32;
    space_ship.rotation += rotation * 3.0 * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::KeyW) {
        **velocity +=
            space_ship.rot_quat() * Vec3::new(0.0, 1.0, 0.0) * 300.0 * 1.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        let brake = if velocity.length() < 50.0 {
            **velocity * 0.99 * time.delta_seconds()
        } else {
            **velocity * 0.9 * time.delta_seconds()
        };
        **velocity -= brake;
    }
    if velocity.length() > 300.0 {
        **velocity = velocity.normalize() * 300.0;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn cleanup(mut commands: Commands) {}
