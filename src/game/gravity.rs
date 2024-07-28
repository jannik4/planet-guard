use super::{ApplyVelocity, Velocity};
use crate::AppState;
use bevy::prelude::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(
            Update,
            apply_gravity
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct Mass(pub f32);

#[derive(Debug, Clone, Copy, Deref, DerefMut, Component)]
pub struct GravityMultiplier(pub f32);

fn apply_gravity(
    time: Res<Time>,
    mut objects: Query<(&mut Velocity, &Transform, Option<&GravityMultiplier>), Without<Mass>>,
    mut masses: Query<(&Mass, &Transform), Without<Velocity>>,
) {
    for (mut velocity, transform, gravity_multiplier) in &mut objects {
        let multiplier = gravity_multiplier.map_or(1.0, |multiplier| multiplier.0);

        for (mass, mass_transform) in &mut masses {
            let direction = mass_transform.translation - transform.translation;
            let distance = direction.length();
            let force = f32::min(multiplier * **mass / distance.powi(2), MAX_FORCE);

            **velocity += direction.normalize() * force * time.delta_seconds();
        }
    }
}

const MAX_FORCE: f32 = 1000.0;
