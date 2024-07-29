use super::{GameState, Home, Planet};
use crate::AppState;
use bevy::prelude::*;

pub struct ShowHomeProgressPlugin;

impl Plugin for ShowHomeProgressPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(
            Update,
            spawn
                .run_if(in_state(AppState::Game))
                .run_if(not(in_state(GameState::GameOver))),
        );
        app.add_systems(Update, despawn.run_if(in_state(AppState::Game)));
    }
}

#[derive(Debug, Resource)]
struct NextProgress(u32);

#[derive(Debug, Component)]
struct DespawnAfterTimer(Timer);

fn spawn(
    mut commands: Commands,
    mut next_progress: ResMut<NextProgress>,
    homes: Query<(&Planet, &Transform), With<Home>>,
) {
    let Ok((planet, transform)) = homes.get_single() else {
        return;
    };

    if (planet.orbit_progress * 100.0) as u32 >= next_progress.0 && next_progress.0 <= 100 {
        commands.spawn((
            Text2dBundle {
                transform: Transform {
                    translation: transform.translation + 50.0 * transform.translation.normalize(),
                    ..default()
                },
                text: Text::from_section(
                    format!("{}%", next_progress.0),
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                ),
                ..default()
            },
            DespawnAfterTimer(Timer::from_seconds(1.5, TimerMode::Once)),
        ));
        next_progress.0 += 25;
    }
}

fn despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Text, &mut DespawnAfterTimer)>,
) {
    for (entity, mut text, mut despawn_after_timer) in &mut query {
        text.sections[0]
            .style
            .color
            .set_alpha(despawn_after_timer.0.fraction_remaining());
        if despawn_after_timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(NextProgress(25));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<NextProgress>();
}
