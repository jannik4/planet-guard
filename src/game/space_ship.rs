use super::{
    ApplyVelocity, BulletBundle, BulletMissileLock, Collider, KeepInMap, MaxVelocity, Velocity,
};
use crate::{
    assets::{AudioAssets, GameAssets},
    AppState,
};
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
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
    material: Handle<ColorMaterial>,
    bullet_material: Handle<ColorMaterial>,
    pub throttle: bool,
    pub brake: bool,
    pub steering: Steering,
    pub shoot: Option<f32>,
    pub shoot_missile_lock: Option<Entity>,
}

impl SpaceShip {
    pub fn material(&self) -> Handle<ColorMaterial> {
        self.material.clone()
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
    pub audio: AudioBundle,
}

impl SpaceShipBundle {
    pub fn new(
        collider_group: u32,
        velocity: Velocity,
        position: Vec3,
        rotation: f32,
        material: Handle<ColorMaterial>,
        bullet_material: Handle<ColorMaterial>,

        audio_assets: &AudioAssets,
        assets: &GameAssets,
    ) -> Self {
        let space_ship = SpaceShip {
            rotation,
            material: material.clone(),
            bullet_material,
            throttle: false,
            brake: false,
            steering: Steering::None,
            shoot: None,
            shoot_missile_lock: None,
        };
        Self {
            collider: Collider {
                radius: 12.0,
                group: collider_group,
            },
            velocity,
            max_velocity: MaxVelocity(180.0),
            keep_in_map: KeepInMap,
            mesh: MaterialMesh2dBundle {
                mesh: assets.space_ship_mesh.clone(),
                material,
                transform: Transform::from_translation(position)
                    .with_rotation(space_ship.rot_quat()),

                ..default()
            },
            audio: AudioBundle {
                source: audio_assets.thruster_fire_000.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::new(0.0),
                    ..default()
                },
            },
            space_ship,
        }
    }
}

fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut space_ships: Query<(
        &Collider,
        &mut SpaceShip,
        &mut Velocity,
        &mut Transform,
        Option<&AudioSink>,
    )>,

    audio_assets: Res<AudioAssets>,
    assets: Res<GameAssets>,
) {
    for (collider, mut space_ship, mut velocity, mut transform, audio) in &mut space_ships {
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

        if let Some(damage) = space_ship.shoot.take() {
            let mut cmds = commands.spawn((
                BulletBundle::new(
                    collider.group ^ u32::MAX,
                    damage,
                    20.0,
                    Velocity(space_ship.rot_quat() * Vec3::new(0.0, 256.0, 0.0)),
                    transform.translation + space_ship.rot_quat() * Vec3::new(0.0, 10.0, 0.0),
                    space_ship.bullet_material.clone(),
                    &assets,
                ),
                StateScoped(AppState::Game),
            ));
            if let Some(target) = space_ship.shoot_missile_lock {
                cmds.insert(BulletMissileLock { target });
            }
            commands.spawn(AudioBundle {
                source: if collider.group & 0b1 != 0 {
                    audio_assets.laser_small_001.clone()
                } else {
                    audio_assets.laser_small_002.clone()
                },
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            });
        }

        transform.rotation = space_ship.rot_quat();

        if let Some(audio) = audio {
            let volume = if collider.group & 0b1 != 0 {
                // Player
                if space_ship.throttle {
                    0.4
                } else {
                    0.0
                }
            } else {
                // Enemy
                if space_ship.throttle {
                    0.15
                } else {
                    0.0
                }
            };

            if audio.volume() != volume {
                audio.set_volume(volume);
            }
        }
    }
}
