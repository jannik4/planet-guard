use super::{ApplyVelocity, GravityMultiplier, Velocity};
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
        damage: f32,
        velocity: Velocity,
        position: Vec3,
        color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            bullet: Bullet { damage },
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
    mut space_ships: Query<(Entity, &Bullet, &Velocity, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, _space_ship, velocity, mut transform) in &mut space_ships {
        transform.rotation = rot_from_velocity(**velocity);

        if transform.translation.length() > 1024.0 {
            commands.entity(entity).despawn();
        }
    }
}
