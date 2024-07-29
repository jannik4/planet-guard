use crate::{
    game::{GameState, Level},
    ui, AppState,
};
use bevy::prelude::*;

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

#[derive(Debug, Component)]
enum ButtonAction {
    Easy,
    Medium,
    Hard,
}

fn update(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_state_game: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::Easy => {
                    commands.insert_resource(Level::easy());
                    next_state.set(AppState::Game);
                    next_state_game.set(GameState::Running);
                }
                ButtonAction::Medium => {
                    commands.insert_resource(Level::medium());
                    next_state.set(AppState::Game);
                    next_state_game.set(GameState::Running);
                }
                ButtonAction::Hard => {
                    commands.insert_resource(Level::hard());
                    next_state.set(AppState::Game);
                    next_state_game.set(GameState::Running);
                }
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateScoped(AppState::MainMenu)));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(AppState::MainMenu),
        ))
        .with_children(|parent| {
            ui::spawn_button_with(parent, "Easy", ButtonAction::Easy);
            ui::spawn_button_with(parent, "Medium", ButtonAction::Medium);
            ui::spawn_button_with(parent, "Hard", ButtonAction::Hard);
        });
}

fn cleanup(mut _commands: Commands) {}
