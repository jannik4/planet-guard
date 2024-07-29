mod background;
mod bullet;
mod enemy;
mod explosion;
mod game_ui;
mod gravity;
mod health;
mod planet;
mod player;
mod space_ship;
mod star;
mod velocity;

use self::{
    bullet::*,
    explosion::*,
    gravity::*,
    health::*, // enemy::*,
    planet::*,
    player::*,
    space_ship::*,
    star::*,
    velocity::*,
};
use crate::{assets::GameAssets, AppState};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
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
            explosion::ExplosionPlugin,
            game_ui::GameUiPlugin,
            background::BackgroundPlugin,
        ));
    }
}

#[derive(Debug, Component)]
pub struct Collider {
    pub radius: f32,
    pub group: u32,
}

#[derive(Debug, Component)]
pub struct Home;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<GameAssets>,
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
        PlayerBundle::new(Vec3::new(10.0, 100.0, 0.0), 0.0, &assets),
        StateScoped(AppState::Game),
    ));

    // Star
    commands.spawn((
        StarBundle::new(
            Mass(200_000.0),
            Color::srgb(4.0, 4.0, 0.8),
            &assets,
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
            materials.add(Color::srgb(2.0, 1.5, 0.2)),
            &assets,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            300.0,
            20.0,
            0.0,
            Mass(100_000.0),
            assets.home_planet_material.clone(),
            &assets,
        ),
        Home,
        Health::new(1000.0),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            450.0,
            40.0,
            0.8,
            Mass(100_000.0),
            materials.add(Color::srgb(1.8, 0.4, 0.9)),
            &assets,
        ),
        StateScoped(AppState::Game),
    ));
}

fn cleanup(mut _commands: Commands) {}
