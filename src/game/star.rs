use super::{gravity::Mass, Collider};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Component)]
pub struct Star;

#[derive(Bundle)]
pub struct StarBundle {
    pub star: Star,
    pub collider: Collider,
    pub mass: Mass,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl StarBundle {
    pub fn new(
        mass: Mass,
        color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            star: Star,
            collider: Collider {
                radius: 16.0,
                group: u32::MAX,
            },
            mass,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(16.0)).into(),
                material: materials.add(color),
                ..default()
            },
        }
    }
}
