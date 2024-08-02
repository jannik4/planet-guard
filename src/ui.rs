use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buttons);
    }
}

fn buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn spawn_button_with<C: Component>(
    parent: &mut ChildBuilder,
    text: impl Into<String>,
    component: C,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.35, 0.35, 0.35)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            component,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        })
        .id()
}
