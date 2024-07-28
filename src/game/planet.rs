use super::gravitiy::Mass;
use crate::AppState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(Update, update.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Component)]
pub struct Planet {
    pub orbit_radius: f32,
    pub orbit_time: f32,
    pub orbit_progress: f32,
}

impl Planet {
    fn position(&self) -> Vec3 {
        let x = self.orbit_radius * self.orbit_progress.cos();
        let y = self.orbit_radius * self.orbit_progress.sin();
        Vec3::new(x, y, 0.0)
    }
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub mass: Mass,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl PlanetBundle {
    pub fn new(
        orbit_radius: f32,
        orbit_time: f32,
        orbit_progress: f32,
        mass: Mass,
        color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let planet = Planet {
            orbit_radius,
            orbit_time,
            orbit_progress,
        };

        Self {
            mass,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(8.0)).into(),
                material: materials.add(color),
                transform: Transform::from_translation(planet.position()),
                ..default()
            },
            planet,
        }
    }
}

fn update(mut planets: Query<(&mut Planet, &mut Transform)>, time: Res<Time>) {
    for (mut planet, mut transform) in &mut planets {
        planet.orbit_progress += time.delta_seconds() / planet.orbit_time;
        transform.translation = planet.position();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn cleanup(mut commands: Commands) {}
