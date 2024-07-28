use crate::AppState;
use bevy::prelude::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(Update, update.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct Mass(pub f32);

#[derive(Debug, Deref, DerefMut, Component)]
pub struct Velocity(pub Vec3);

fn update(
    time: Res<Time>,
    mut objects: Query<(&mut Velocity, &mut Transform), Without<Mass>>,
    mut masses: Query<(&Mass, &Transform), Without<Velocity>>,
) {
    for (mut velocity, mut transform) in &mut objects {
        for (mass, mass_transform) in &mut masses {
            let direction = mass_transform.translation - transform.translation;
            let distance = direction.length();
            let force = direction.normalize() * **mass / distance.powi(2);
            **velocity += force * time.delta_seconds();
        }

        transform.translation += **velocity * time.delta_seconds();
        if transform.translation.length() > 512.0 {
            transform.translation = transform.translation.normalize() * 512.0;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn cleanup(mut commands: Commands) {}
