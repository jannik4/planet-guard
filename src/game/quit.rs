use super::ApplyVelocity;
use crate::AppState;
use bevy::prelude::*;

pub struct QuitPlugin;

impl Plugin for QuitPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        // Update
        app.add_systems(
            Update,
            update.after(ApplyVelocity).run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Debug, Resource)]
struct QuitTimer(Timer);

#[derive(Debug, Component)]
struct QuitUi;

#[derive(Debug, Component)]
struct QuitUiText;

#[derive(Debug, Component)]
struct QuitUiProgress;

fn update(
    mut quit_timer: ResMut<QuitTimer>,
    mut quit_ui: Query<&mut Visibility, With<QuitUi>>,
    mut quit_ui_text: Query<&mut Text, With<QuitUiText>>,
    mut quit_ui_progress: Query<&mut Style, With<QuitUiProgress>>,

    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(mut visibility) = quit_ui.get_single_mut() else {
        return;
    };
    let Ok(mut text) = quit_ui_text.get_single_mut() else {
        return;
    };
    let Ok(mut style) = quit_ui_progress.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Escape) || keyboard_input.pressed(KeyCode::Backspace) {
        if quit_timer.0.tick(time.delta()).just_finished() {
            next_state.set(AppState::MainMenu);
        } else {
            *visibility = Visibility::Inherited;
            text.sections[0].value = if keyboard_input.pressed(KeyCode::Escape) {
                "Hold ESC to quit".to_string()
            } else {
                "Hold BACKSPACE to quit".to_string()
            };
            style.width = Val::Px(300.0 * quit_timer.0.fraction_remaining());
        }
    } else {
        quit_timer.0.reset();
        *visibility = Visibility::Hidden;
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(QuitTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(60.0),
                    top: Val::Percent(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            QuitUi,
            StateScoped(AppState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                QuitUiText,
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(5.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                },
                QuitUiProgress,
            ));
        });
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<QuitTimer>();
}
