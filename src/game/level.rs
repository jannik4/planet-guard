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
        app.add_systems(
            Update,
            home_laser
                .after(check_if_won)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::GameWon)),
        );
    }
}

#[derive(Debug, Resource)]
struct HomeLaser {
    timer: Timer,
    enemies: Vec<Transform>,
}

fn home_laser(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut home_laser: ResMut<HomeLaser>,
    homes: Query<&Transform, With<Home>>,
) {
    if home_laser.timer.tick(time.delta()).finished() {
        return;
    }

    let Ok(home_transform) = homes.get_single() else {
        return;
    };

    let factor = home_laser.timer.fraction_remaining().powf(0.1);
    let color = Color::srgb(5.0 * factor, 0.1 * factor, 0.1 * factor);

    for enemy_transform in &home_laser.enemies {
        gizmos.line_2d(
            home_transform.translation.xy(),
            enemy_transform.translation.xy(),
            color,
        );
    }
}

fn check_if_won(
    mut commands: Commands,
    mut home_laser: ResMut<HomeLaser>,
    homes: Query<(&Planet, &Health), (With<Home>, Without<Enemy>)>,
    mut enemies: Query<(&mut Health, &Transform), With<Enemy>>,
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

        for (mut enemy_health, enemy_transform) in &mut enemies {
            enemy_health.set_dead();
            home_laser.enemies.push(*enemy_transform);
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

fn setup(mut commands: Commands) {
    commands.insert_resource(HomeLaser {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
        enemies: Vec::new(),
    });
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<Level>();
    commands.remove_resource::<HomeLaser>();
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
    pub enemy_force_to_home_probability: f32,
}

impl Level {
    pub fn easy() -> Self {
        Self {
            name: "Easy".to_string(),

            home_orbit_time: 30.0,

            home_health: Health::new(300.0),
            player_health: Health::new(50.0),
            enemy_health: Health::new(10.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 5.0,
            enemy_force_to_home_probability: 0.1,
        }
    }

    pub fn medium() -> Self {
        Self {
            name: "Medium".to_string(),

            home_orbit_time: 60.0,

            home_health: Health::new(400.0),
            player_health: Health::new(30.0),
            enemy_health: Health::new(20.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 5.0,
            enemy_force_to_home_probability: 0.15,
        }
    }

    pub fn hard() -> Self {
        Self {
            name: "Hard".to_string(),

            home_orbit_time: 90.0,

            home_health: Health::new(500.0),
            player_health: Health::new(20.0),
            enemy_health: Health::new(20.0),

            player_damage: 10.0,
            enemy_damage: 10.0,

            enemy_spawn_interval: 4.0,
            enemy_force_to_home_probability: 0.2,
        }
    }
}
