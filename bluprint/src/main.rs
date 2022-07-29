#![feature(let_chains)]
mod world;
mod resources;
mod scenes;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;
use world::WorldPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Editor,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldPlugin)
        .add_loopless_state(AppState::MainMenu)
        .add_plugin(scenes::menu::MenuPlugin)
        .add_plugin(scenes::editor::EditorPlugin)
        .run();
}
