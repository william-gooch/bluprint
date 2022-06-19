mod scenes;
mod resources;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use resources::map_file::MapFileRes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Editor,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_loopless_state(AppState::MainMenu)
        .insert_resource(MapFileRes::default())
        .add_plugin(scenes::menu::MenuPlugin)
        .add_plugin(scenes::editor::EditorPlugin)
        .run();
}
