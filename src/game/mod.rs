mod background;
mod bullet;
mod enemy;
mod gravity;
mod planet;
mod player;
mod space_ship;
mod star;
mod velocity;

use self::{
    bullet::*, enemy::*, gravity::*, planet::*, player::*, space_ship::*, star::*, velocity::*,
};
use crate::AppState;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
    sprite::MaterialMesh2dBundle,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        app.add_plugins((
            velocity::VelocityPlugin,
            gravity::GravityPlugin,
            star::StarPlugin,
            planet::PlanetPlugin,
            bullet::BulletPlugin,
            space_ship::SpaceShipPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            background::BackgroundPlugin,
        ));
    }
}

#[derive(Debug, Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Debug, Component)]
pub struct Health(pub f32);

#[derive(Debug, Component)]
pub struct Star;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::FixedVertical(1024.0),
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        SpaceShipBundle::new(
            Velocity(Vec3::ZERO),
            Vec3::new(10.0, 100.0, 0.0),
            0.0,
            Color::srgb(0.6, 0.6, 1.4),
            Color::srgb(0.0, 0.0, 2.0),
            &mut meshes,
            &mut materials,
        ),
        Player,
        StateScoped(AppState::Game),
    ));

    for i in 0..10 {
        let alpha = (i as f32 / 10.0) * std::f32::consts::TAU;
        commands.spawn((
            EnemyBundle::new(
                Vec3::new(f32::cos(alpha) * 512.0, f32::sin(alpha) * 512.0, 0.0),
                alpha + std::f32::consts::FRAC_PI_2,
                &mut meshes,
                &mut materials,
            ),
            StateScoped(AppState::Game),
        ));
    }

    // Star
    commands.spawn((
        StarBundle::new(
            Mass(200_000.0),
            Color::srgb(4.0, 4.0, 0.8),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            150.0,
            10.0,
            0.5,
            Mass(100_000.0),
            Color::srgb(2.0, 1.5, 0.2),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            300.0,
            20.0,
            0.0,
            Mass(100_000.0),
            // Color::srgb(0.2, 2.0, 0.5),
            Color::srgb(0.2, 0.5, 2.0),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            450.0,
            40.0,
            0.8,
            Mass(100_000.0),
            // Color::srgb(0.2, 0.5, 2.0),
            Color::srgb(1.8, 0.4, 0.9),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));
}

fn cleanup(mut commands: Commands) {}
