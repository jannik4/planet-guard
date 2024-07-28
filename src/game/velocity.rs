use crate::AppState;
use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            apply_velocity
                .in_set(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ApplyVelocity;

#[derive(Debug, Clone, Copy, Deref, DerefMut, Component)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Clone, Copy, Deref, DerefMut, Component)]
pub struct MaxVelocity(pub f32);

#[derive(Debug, Component)]
pub struct KeepInMap;

fn apply_velocity(
    time: Res<Time>,
    mut objects: Query<(
        &mut Velocity,
        &mut Transform,
        Option<&MaxVelocity>,
        Has<KeepInMap>,
    )>,
) {
    for (mut velocity, mut transform, max_velocity, keep_in_map) in &mut objects {
        if let Some(max_velocity) = max_velocity {
            if velocity.length() > **max_velocity {
                **velocity = velocity.normalize() * **max_velocity;
            }
        }

        transform.translation += **velocity * time.delta_seconds();

        if keep_in_map && transform.translation.length() > 512.0 {
            transform.translation = transform.translation.normalize() * 512.0;

            let sub_velocity = transform.translation + **velocity
                - (transform.translation + **velocity).normalize() * 512.0;
            **velocity -= sub_velocity;
        }
    }
}
