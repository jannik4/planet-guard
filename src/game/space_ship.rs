use super::{ApplyVelocity, KeepInMap, MaxVelocity, Velocity};
use crate::AppState;
use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    sprite::MaterialMesh2dBundle,
};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

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
pub struct SpaceShip {
    pub rotation: f32,
}

impl SpaceShip {
    pub fn rot_quat(&self) -> Quat {
        Quat::from_rotation_z(self.rotation)
    }
}

#[derive(Bundle)]
pub struct SpaceShipBundle {
    pub space_ship: SpaceShip,
    pub velocity: Velocity,
    pub max_velocity: MaxVelocity,
    pub keep_in_map: KeepInMap,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl SpaceShipBundle {
    pub fn new(
        velocity: Velocity,
        position: Vec3,
        rotation: f32,
        color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let space_ship = SpaceShip { rotation };
        Self {
            velocity,
            max_velocity: MaxVelocity(300.0),
            keep_in_map: KeepInMap,
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(mesh()).into(),
                material: materials.add(color),
                transform: Transform::from_translation(position)
                    .with_rotation(space_ship.rot_quat()),

                ..default()
            },
            space_ship,
        }
    }
}

fn mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            //
            [0.0, 10.0, 0.0],
            [5.0, 0.0, 0.0],
            [-5.0, 0.0, 0.0],
            //
            [-5.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [-10.0, -10.0, 0.0],
            //
            [0.0, 0.0, 0.0],
            [5.0, 0.0, 0.0],
            [10.0, -10.0, 0.0],
        ],
    )
}

fn update(mut space_ships: Query<(&SpaceShip, &mut Transform)>) {
    for (space_ship, mut transform) in &mut space_ships {
        transform.rotation = space_ship.rot_quat();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn cleanup(mut commands: Commands) {}
