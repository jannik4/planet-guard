use super::{
    ApplyVelocity, Health, Player, SpaceShip, SpaceShipBundle, Steering, UpdateSpaceShip, Velocity,
};
use crate::AppState;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(
            Update,
            update
                .before(UpdateSpaceShip)
                .before(ApplyVelocity)
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Component)]
pub struct Enemy {
    last_shot: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self { last_shot: 0.0 }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub space_ship: SpaceShipBundle,
}

impl EnemyBundle {
    pub fn new(
        position: Vec3,
        rotation: f32,

        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            enemy: Enemy::new(),
            health: Health(100.0),
            space_ship: SpaceShipBundle::new(
                Velocity(Vec3::ZERO),
                position,
                rotation,
                Color::srgb(1.4, 0.6, 0.6),
                Color::srgb(2.0, 0.0, 0.0),
                meshes,
                materials,
            ),
        }
    }
}

fn update(
    time: Res<Time>,
    mut enemies: Query<(&Transform, &mut SpaceShip, &mut Enemy), Without<Player>>,
    players: Query<&Transform, With<Player>>,
) {
    let Ok(player) = players.get_single() else {
        return;
    };

    for (transform, mut space_ship, mut enemy) in &mut enemies {
        let direction = player.translation - transform.translation;
        let distance = direction.length();
        let direction = direction.normalize();
        let angle_between = direction
            .angle_between(space_ship.rot_quat() * Vec3::Y)
            .to_degrees();

        space_ship.steering = match angle_between {
            angle if angle > 5.0 => Steering::Right,
            angle if angle < -5.0 => Steering::Left,
            _ => Steering::None,
        };
        space_ship.throttle = distance > 100.0;
        space_ship.brake = distance < 50.0;
        space_ship.shoot = distance < 200.0
            && angle_between.abs() < 10.0
            && time.elapsed_seconds() - enemy.last_shot > 0.5;

        if space_ship.shoot {
            enemy.last_shot = time.elapsed_seconds();
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
