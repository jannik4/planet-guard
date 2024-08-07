use super::{gravity::Mass, Collider};
use crate::{assets::GameAssets, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
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
        let x = self.orbit_radius * (self.orbit_progress * std::f32::consts::TAU).cos();
        let y = self.orbit_radius * (self.orbit_progress * std::f32::consts::TAU).sin();
        Vec3::new(x, y, 0.0)
    }
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub collider: Collider,
    pub mass: Mass,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl PlanetBundle {
    pub fn new(
        orbit_radius: f32,
        orbit_time: f32,
        orbit_progress: f32,
        mass: Mass,
        material: Handle<ColorMaterial>,

        assets: &GameAssets,
    ) -> Self {
        let planet = Planet {
            orbit_radius,
            orbit_time,
            orbit_progress,
        };

        Self {
            collider: Collider {
                radius: 8.0,
                group: 0b100,
            },
            mass,
            mesh: MaterialMesh2dBundle {
                mesh: assets.planet_mesh.clone(),
                material,
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
