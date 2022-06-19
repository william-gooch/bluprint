use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuComponent;

fn draw_ui(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Main Menu")
        .fixed_pos(egui::pos2(100.0, 100.0))
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Hello, world!");

            if ui.button("Open Editor").clicked() {
                commands.insert_resource(NextState(AppState::Editor));
            }
        });
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_system(
                draw_ui
                    .run_in_state(AppState::MainMenu)
            );
    }
}
