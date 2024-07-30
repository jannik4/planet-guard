#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::AppExit;

fn main() -> AppExit {
    planet_guard::build_app().run()
}
