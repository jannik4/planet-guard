use super::{GravityMultiplier, Velocity};
use crate::{
    assets::{AudioAssets, GameAssets},
    AppState,
};
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosion>();

        // Update
        app.add_systems(
            Update,
            (spawn_explosions, update_explosions, update_particles)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Event)]
pub struct SpawnExplosion {
    pub position: Vec3,
    pub material: Handle<ColorMaterial>,
}

#[derive(Debug, Component)]
struct Explosion {
    timer: Timer,
}

#[derive(Debug, Component)]
struct Particle;

fn rot_from_velocity(velocity: Vec3) -> Quat {
    Quat::from_rotation_z(f32::atan2(velocity.y, velocity.x))
}

fn spawn_explosions(
    mut events: EventReader<SpawnExplosion>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    assets: Res<GameAssets>,
) {
    let mut volume = 0.5;
    for spawn in events.read() {
        commands
            .spawn((
                SpatialBundle::default(),
                Explosion {
                    timer: Timer::from_seconds(0.3, TimerMode::Once),
                },
                StateScoped(AppState::Game),
            ))
            .with_children(|builder| {
                for i in 0..10 {
                    let alpha = (i as f32 / 10.0) * std::f32::consts::TAU;
                    let velocity =
                        Velocity(Vec3::new(f32::cos(alpha), f32::sin(alpha), 0.0) * 64.0);
                    builder.spawn((
                        velocity,
                        GravityMultiplier(10.0),
                        MaterialMesh2dBundle {
                            mesh: assets.explosion_mesh.clone(),
                            material: spawn.material.clone(),
                            transform: Transform::from_translation(spawn.position)
                                .with_rotation(rot_from_velocity(*velocity)),

                            ..default()
                        },
                    ));
                }
            });
        commands.spawn(AudioBundle {
            source: audio_assets.explosion_crunch_000.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(volume),
                speed: 2.0,
                ..default()
            },
        });

        volume *= 0.5;
    }
}

fn update_explosions(
    time: Res<Time>,
    mut explosions: Query<(Entity, &mut Explosion)>,
    mut commands: Commands,
) {
    for (entity, mut explosion) in &mut explosions {
        if explosion.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_particles(mut particles: Query<(&Velocity, &mut Transform), With<Particle>>) {
    for (velocity, mut transform) in &mut particles {
        transform.rotation = rot_from_velocity(**velocity);
    }
}
