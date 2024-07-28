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
    mut bullets: Query<(Entity, &Bullet, &Velocity, &mut Transform)>,
    mut objects: Query<(&Transform, &Collider, Option<&mut Health>), Without<Bullet>>,
    mut commands: Commands,
) {
    for (entity, bullet, velocity, mut transform) in &mut bullets {
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
