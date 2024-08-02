use super::{ApplyVelocity, Collider, GameState, GravityMultiplier, Health, Velocity};
use crate::{
    assets::{AudioAssets, GameAssets},
    AppState,
};
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            update
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Component)]
pub struct BulletMissileLock {
    pub target: Entity,
}

#[derive(Debug, Component)]
pub struct Bullet {
    pub collider_filter: u32,
    pub damage: f32,
    pub time_to_live: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub velocity: Velocity,
    pub gravity_multiplier: GravityMultiplier,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl BulletBundle {
    pub fn new(
        collider_filter: u32,
        damage: f32,
        time_to_live: f32,
        velocity: Velocity,
        position: Vec3,
        material: Handle<ColorMaterial>,
        assets: &GameAssets,
    ) -> Self {
        Self {
            bullet: Bullet {
                collider_filter,
                damage,
                time_to_live,
            },
            velocity,
            gravity_multiplier: GravityMultiplier(10.0),
            mesh: MaterialMesh2dBundle {
                mesh: assets.bullet_mesh.clone(),
                material,
                transform: Transform::from_translation(position)
                    .with_rotation(rot_from_velocity(*velocity)),

                ..default()
            },
        }
    }
}

fn rot_from_velocity(velocity: Vec3) -> Quat {
    Quat::from_rotation_z(f32::atan2(velocity.y, velocity.x))
}

fn update(
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    mut bullets: Query<(
        Entity,
        &mut Bullet,
        Option<&BulletMissileLock>,
        &mut Velocity,
        &mut Transform,
    )>,
    mut objects: Query<(&Transform, &Collider, Option<&mut Health>), Without<Bullet>>,
    targets: Query<&Transform, Without<Bullet>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, mut bullet, lock, mut velocity, mut transform) in &mut bullets {
        bullet.time_to_live -= time.delta_seconds();
        if let Some(lock) = lock {
            if let Ok(target) = targets.get(lock.target) {
                let target_velocity =
                    (target.translation - transform.translation).normalize() * velocity.length();
                **velocity = Vec3::lerp(
                    **velocity,
                    target_velocity,
                    1.0 - f32::exp(f32::ln(0.95) * 60.0 * time.delta_seconds()),
                );
            }
        }
        transform.rotation = rot_from_velocity(**velocity);

        let mut despawn = transform.translation.length() > 1024.0 || bullet.time_to_live <= 0.0;

        for (obj_transform, obj_collider, obj_health) in &mut objects {
            if obj_collider.group & bullet.collider_filter == 0 {
                continue;
            }
            if Vec3::distance_squared(transform.translation, obj_transform.translation)
                <= obj_collider.radius * obj_collider.radius
            {
                if **game_state == GameState::Running {
                    if let Some(mut health) = obj_health {
                        if !(obj_collider.group & 0b100 != 0 && bullet.collider_filter & 0b1 == 0) {
                            health.damage(bullet.damage);
                        }

                        commands.spawn(AudioBundle {
                            source: audio_assets.impact_metal_004.clone(),
                            settings: PlaybackSettings {
                                mode: PlaybackMode::Despawn,
                                volume: Volume::new(0.35),
                                speed: 4.0,
                                ..default()
                            },
                        });
                    }
                }
                despawn = true;
                break;
            }
        }

        if despawn {
            commands.entity(entity).despawn();
        }
    }
}
