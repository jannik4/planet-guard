use super::{enemy::Enemy, Bullet, GameState, Health, Home, Planet};
use crate::AppState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        app.add_systems(
            Update,
            check_if_won
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
    }
}

fn check_if_won(
    mut commands: Commands,
    homes: Query<(&Planet, &Health), (With<Home>, Without<Enemy>)>,
    mut enemies: Query<&mut Health, With<Enemy>>,
    mut _bullets: Query<&mut Bullet>,
    mut next_state_game: ResMut<NextState<GameState>>,
) {
    let Ok((home_planet, home_health)) = homes.get_single() else {
        return;
    };

    if home_health.current() <= 0.0 {
        next_state_game.set(GameState::GameOver);
        spawn_text(
            &mut commands,
            "YOU LOSE!".to_string(),
            Color::srgb(8.0, 0.6, 0.6),
        );
    } else if home_planet.orbit_progress >= 1.0 {
        next_state_game.set(GameState::GameWon);
        spawn_text(
            &mut commands,
            "YOU WIN!".to_string(),
            Color::srgb(0.9, 8.0, 0.9),
        );

        for mut enemy_health in &mut enemies {
            enemy_health.set_dead();
        }
    }
}

fn spawn_text(commands: &mut Commands, text: String, color: Color) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::Game),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 80.0,
                    color,
                    ..default()
                },
            ));
        });
}

fn setup() {}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<Level>();
}

#[derive(Debug, Resource)]
pub struct Level {
    #[allow(dead_code)]
    pub name: String,

    pub home_orbit_time: f32,

    pub home_health: Health,
    pub player_health: Health,
    pub enemy_health: Health,

    pub player_damage: f32,
    pub enemy_damage: f32,

    pub enemy_spawn_interval: f32,
}

impl Level {
    pub fn easy() -> Self {
        Self {
            name: "Easy".to_string(),

            home_orbit_time: 30.0,

            home_health: Health::new(200.0),
            player_health: Health::new(50.0),
            enemy_health: Health::new(10.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 5.0,
        }
    }

    pub fn medium() -> Self {
        Self {
            name: "Medium".to_string(),

            home_orbit_time: 60.0,

            home_health: Health::new(300.0),
            player_health: Health::new(30.0),
            enemy_health: Health::new(20.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 5.0,
        }
    }

    pub fn hard() -> Self {
        Self {
            name: "Hard".to_string(),

            home_orbit_time: 90.0,

            home_health: Health::new(300.0),
            player_health: Health::new(20.0),
            enemy_health: Health::new(30.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 5.0,
        }
    }
}
