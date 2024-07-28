use super::{ApplyVelocity, Collider, GravityMultiplier, Health, Velocity};
use crate::AppState;
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
        velocity: Velocity,
        position: Vec3,
        color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            bullet: Bullet {
                collider_filter,
                damage,
            },
            velocity,
            gravity_multiplier: GravityMultiplier(10.0),
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(6.0, 2.0)).into(),
                material: materials.add(color),
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
    mut bullets: Query<(
        Entity,
        &Bullet,
        Option<&BulletMissileLock>,
        &mut Velocity,
        &mut Transform,
    )>,
    mut objects: Query<(&Transform, &Collider, Option<&mut Health>), Without<Bullet>>,
    targets: Query<&Transform, Without<Bullet>>,
    mut commands: Commands,
) {
    for (entity, bullet, lock, mut velocity, mut transform) in &mut bullets {
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

        let mut despawn = transform.translation.length() > 1024.0;

        for (obj_transform, obj_collider, obj_health) in &mut objects {
            if obj_collider.group & bullet.collider_filter == 0 {
                continue;
            }
            if Vec3::distance_squared(transform.translation, obj_transform.translation)
                <= obj_collider.radius * obj_collider.radius
            {
                if let Some(mut health) = obj_health {
                    health.0 -= bullet.damage;
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
