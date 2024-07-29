use super::Health;
use crate::AppState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);
    }
}

#[derive(Debug, Resource)]
pub struct Level {
    pub name: String,

    pub home_orbit_time: f32,

    pub home_health: Health,
    pub player_health: Health,
    pub enemy_health: Health,

    pub player_damage: f32,
    pub enemy_damage: f32,
}

impl Level {
    pub fn easy() -> Self {
        Self {
            name: "Easy".to_string(),

            home_orbit_time: 15.0,

            home_health: Health::new(1000.0),
            player_health: Health::new(50.0),
            enemy_health: Health::new(10.0),

            player_damage: 10.0,
            enemy_damage: 10.0,
        }
    }

    pub fn medium() -> Self {
        Self {
            name: "Medium".to_string(),

            home_orbit_time: 30.0,

            home_health: Health::new(500.0),
            player_health: Health::new(50.0),
            enemy_health: Health::new(20.0),

            player_damage: 10.0,
            enemy_damage: 10.0,
        }
    }

    pub fn hard() -> Self {
        Self {
            name: "Hard".to_string(),

            home_orbit_time: 60.0,

            home_health: Health::new(250.0),
            player_health: Health::new(50.0),
            enemy_health: Health::new(30.0),

            player_damage: 10.0,
            enemy_damage: 10.0,
        }
    }
}

fn setup() {}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<Level>();
}
