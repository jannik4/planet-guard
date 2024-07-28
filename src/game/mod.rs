use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        // app.add_systems(Update, update.run_if(in_state(AppState::Game)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateScoped(AppState::Game)));
}

fn cleanup(mut commands: Commands) {}
