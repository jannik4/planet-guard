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
    pub kind: ExplosionKind,
}

#[derive(Debug)]
pub enum ExplosionKind {
    Small,
    Large,
}

impl ExplosionKind {
    fn particle_count(&self) -> usize {
        match self {
            ExplosionKind::Small => 10,
            ExplosionKind::Large => 12,
        }
    }

    fn initial_speed(&self) -> f32 {
        match self {
            ExplosionKind::Small => 64.0,
            ExplosionKind::Large => 40.0,
        }
    }

    fn time_to_live(&self) -> f32 {
        match self {
            ExplosionKind::Small => 0.3,
            ExplosionKind::Large => 0.7,
        }
    }

    fn size(&self) -> f32 {
        match self {
            ExplosionKind::Small => 1.0,
            ExplosionKind::Large => 1.5,
        }
    }

    fn audio_source(&self, audio_assets: &AudioAssets) -> Handle<AudioSource> {
        match self {
            ExplosionKind::Small => audio_assets.explosion_crunch_000.clone(),
            ExplosionKind::Large => audio_assets.low_frequency_explosion_000.clone(),
        }
    }

    fn audio_volume(&self) -> f32 {
        match self {
            ExplosionKind::Small => 0.5,
            ExplosionKind::Large => 2.0,
        }
    }

    fn audio_speed(&self) -> f32 {
        match self {
            ExplosionKind::Small => 2.0,
            ExplosionKind::Large => 1.0,
        }
    }
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
    // The volume of the explosion sound is reduced by 50% for each subsequent explosion in the same frame.
    let mut volume_multiplier = 1.0;

    for spawn in events.read() {
        commands
            .spawn((
                SpatialBundle::default(),
                Explosion {
                    timer: Timer::from_seconds(spawn.kind.time_to_live(), TimerMode::Once),
                },
                StateScoped(AppState::Game),
            ))
            .with_children(|builder| {
                for i in 0..spawn.kind.particle_count() {
                    let alpha =
                        (i as f32 / spawn.kind.particle_count() as f32) * std::f32::consts::TAU;
                    let velocity = Velocity(
                        Vec3::new(f32::cos(alpha), f32::sin(alpha), 0.0)
                            * spawn.kind.initial_speed(),
                    );
                    builder.spawn((
                        velocity,
                        GravityMultiplier(10.0),
                        MaterialMesh2dBundle {
                            mesh: assets.explosion_mesh.clone(),
                            material: spawn.material.clone(),
                            transform: Transform::from_translation(spawn.position)
                                .with_scale(Vec3::splat(spawn.kind.size()))
                                .with_rotation(rot_from_velocity(*velocity)),

                            ..default()
                        },
                    ));
                }
            });
        commands.spawn(AudioBundle {
            source: spawn.kind.audio_source(&audio_assets),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new(volume_multiplier * spawn.kind.audio_volume()),
                speed: spawn.kind.audio_speed(),
                ..default()
            },
        });

        volume_multiplier *= 0.5;
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
