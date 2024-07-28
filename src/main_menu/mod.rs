use crate::AppState;
use bevy::{color::palettes::css::*, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::MainMenu), setup);
        app.add_systems(OnExit(AppState::MainMenu), cleanup);

        // Update
        app.add_systems(Update, update.run_if(in_state(AppState::MainMenu)));
    }
}

fn update(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), LIME);

    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::new(16, 9),
            Vec2::new(80., 80.),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., BLUE),
        (Vec2::new(-255., -155.), RED),
        (Vec2::new(255., -155.), LIME),
        (Vec2::Y * 300., BLUE),
    ]);

    gizmos.rect_2d(Vec2::ZERO, 0., Vec2::splat(650.), BLACK);
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateScoped(AppState::MainMenu)));
}

fn cleanup(mut commands: Commands) {}
