use crate::{assets::GameAssets, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);
    }
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: assets.background_mesh.clone(),
            material: assets.background_material.clone(),
            ..default()
        },
        StateScoped(AppState::Game),
    ));
}

fn cleanup() {}
