mod background;
mod gravitiy;
mod planet;
mod player;
mod space_ship;

use self::{gravitiy::*, planet::*, player::*, space_ship::*};
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
            planet::PlanetPlugin,
            space_ship::SpaceShipPlugin,
            gravitiy::GravityPlugin,
            player::PlayerPlugin,
            background::BackgroundPlugin,
        ));
    }
}

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
            &mut meshes,
            &mut materials,
        ),
        Player,
        StateScoped(AppState::Game),
    ));

    // Star
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(16.0)).into(),
            material: materials.add(Color::srgb(4.0, 4.0, 0.8)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Star,
        Mass(1000.0),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            150.0,
            1.0,
            0.5,
            Mass(100.0),
            Color::srgb(2.0, 1.5, 0.2),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            300.0,
            2.0,
            0.0,
            Mass(100.0),
            Color::srgb(0.2, 2.0, 0.5),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));

    commands.spawn((
        PlanetBundle::new(
            450.0,
            4.0,
            0.8,
            Mass(100.0),
            Color::srgb(0.2, 0.5, 2.0),
            &mut meshes,
            &mut materials,
        ),
        StateScoped(AppState::Game),
    ));
}

fn cleanup(mut commands: Commands) {}
