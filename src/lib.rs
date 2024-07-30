// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod full_screen;
mod game;
mod main_menu;
mod mute;
mod splash_screen;
mod ui;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));

    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_state::<AssetsState>();

    app.add_loading_state({
        LoadingState::new(AssetsState::Loading)
            .continue_to_state(AssetsState::Loaded)
            .on_failure_continue_to_state(AssetsState::Error)
    });

    app.add_plugins((
        assets::GameAssetsPlugin,
        splash_screen::SplashScreenPlugin,
        main_menu::MainMenuPlugin,
        game::GamePlugin,
        ui::UiPlugin,
        full_screen::FullScreenPlugin,
        mute::MutePlugin,
    ));

    app
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    SplashScreen,
    MainMenu,
    Game,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AssetsState {
    #[default]
    Loading,
    Loaded,
    Error,
}
