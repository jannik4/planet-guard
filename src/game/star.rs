use crate::assets::GameAssets;

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

        assets: &GameAssets,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            star: Star,
            collider: Collider {
                radius: 16.0,
                group: 0b100,
            },
            mass,
            mesh: MaterialMesh2dBundle {
                mesh: assets.star_mesh.clone(),
                material: materials.add(color),
                ..default()
            },
        }
    }
}
