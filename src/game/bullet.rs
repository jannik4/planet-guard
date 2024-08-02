use super::{
    ApplyVelocity, Collider, ExplosionKind, GameState, GravityMultiplier, Health, SpawnExplosion,
    Velocity,
};
use crate::{assets::GameAssets, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
        &Handle<ColorMaterial>,
    )>,
    mut objects: Query<(&Transform, &Collider, Option<&mut Health>), Without<Bullet>>,
    targets: Query<&Transform, Without<Bullet>>,
    mut commands: Commands,
    mut explosions: EventWriter<SpawnExplosion>,
) {
    for (entity, mut bullet, lock, mut velocity, mut transform, material) in &mut bullets {
        bullet.time_to_live -= time.delta_seconds();
        if let Some(lock) = lock {
            if let Ok(target) = targets.get(lock.target) {
                let target_velocity =
                    (target.translation - transform.translation).normalize() * velocity.length();
                **velocity = Vec3::lerp(
                    **velocity,
                    target_velocity,
                    1.0 - f32::exp(f32::ln(0.9) * 60.0 * time.delta_seconds()),
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
                        let spawn_explosion = if !(obj_collider.group & 0b100 != 0
                            && bullet.collider_filter & 0b1 == 0)
                        {
                            health.damage(bullet.damage);
                            health.current() > 0.0
                        } else {
                            false
                        };

                        if spawn_explosion {
                            explosions.send(SpawnExplosion {
                                position: (transform.translation + obj_transform.translation) / 2.0
                                    + Vec3::Z,
                                material: material.clone(),
                                kind: ExplosionKind::Small,
                            });
                        }
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
