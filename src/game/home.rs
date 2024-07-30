use super::{ApplyVelocity, ExplosionKind, GameState, Health, SpawnExplosion};
use crate::AppState;
use bevy::prelude::*;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            explode
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Debug, Component)]
pub struct Home;

fn explode(
    mut commands: Commands,
    mut explosions: EventWriter<SpawnExplosion>,
    mut homes: Query<(Entity, &Health, &Transform, &Handle<ColorMaterial>), With<Home>>,
) {
    let Ok((entity, health, transform, material)) = homes.get_single_mut() else {
        return;
    };

    if health.current() <= 0.0 {
        commands.entity(entity).despawn();
        explosions.send(SpawnExplosion {
            position: transform.translation,
            material: material.clone(),
            kind: ExplosionKind::Large,
        });
    }
}
