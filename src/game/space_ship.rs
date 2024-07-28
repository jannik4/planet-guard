use super::{
    ApplyVelocity, BulletBundle, BulletMissileLock, Collider, KeepInMap, MaxVelocity, Velocity,
};
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
                .in_set(UpdateSpaceShip)
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct UpdateSpaceShip;

#[derive(Debug, Component)]
pub struct SpaceShip {
    rotation: f32,
    color: Color,
    bullet_color: Color,
    pub throttle: bool,
    pub brake: bool,
    pub steering: Steering,
    pub shoot: bool,
    pub shoot_missile_lock: Option<Entity>,
}

impl SpaceShip {
    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Debug)]
pub enum Steering {
    Left,
    Right,
    None,
}

impl SpaceShip {
    pub fn rot_quat(&self) -> Quat {
        Quat::from_rotation_z(self.rotation)
    }
}

#[derive(Bundle)]
pub struct SpaceShipBundle {
    pub space_ship: SpaceShip,
    pub collider: Collider,
    pub velocity: Velocity,
    pub max_velocity: MaxVelocity,
    pub keep_in_map: KeepInMap,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl SpaceShipBundle {
    pub fn new(
        collider_group: u32,
        velocity: Velocity,
        position: Vec3,
        rotation: f32,
        color: Color,
        bullet_color: Color,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let space_ship = SpaceShip {
            rotation,
            color,
            bullet_color,
            throttle: false,
            brake: false,
            steering: Steering::None,
            shoot: false,
            shoot_missile_lock: None,
        };
        Self {
            collider: Collider {
                radius: 10.0,
                group: collider_group,
            },
            velocity,
            max_velocity: MaxVelocity(180.0),
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

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut space_ships: Query<(&Collider, &mut SpaceShip, &mut Velocity, &mut Transform)>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (collider, mut space_ship, mut velocity, mut transform) in &mut space_ships {
        space_ship.rotation += match space_ship.steering {
            Steering::Left => 3.0 * time.delta_seconds(),
            Steering::Right => -3.0 * time.delta_seconds(),
            Steering::None => 0.0,
        };

        if space_ship.throttle {
            **velocity += space_ship.rot_quat()
                * Vec3::new(0.0, 1.0, 0.0)
                * 300.0
                * 1.0
                * time.delta_seconds();
        }
        if space_ship.brake {
            let brake = if velocity.length() < 50.0 {
                **velocity * 0.99 * time.delta_seconds()
            } else {
                **velocity * 0.9 * time.delta_seconds()
            };
            **velocity -= brake;
        }

        if space_ship.shoot {
            let mut cmds = commands.spawn(BulletBundle::new(
                collider.group ^ 0b11,
                10.0,
                Velocity(space_ship.rot_quat() * Vec3::new(0.0, 256.0, 0.0)),
                transform.translation + space_ship.rot_quat() * Vec3::new(0.0, 10.0, 0.0),
                space_ship.bullet_color,
                &mut meshes,
                &mut materials,
            ));
            if let Some(target) = space_ship.shoot_missile_lock {
                cmds.insert(BulletMissileLock { target });
            }
            space_ship.shoot = false;
        }

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
