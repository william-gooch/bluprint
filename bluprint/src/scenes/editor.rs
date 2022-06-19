use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::AppState;

pub struct EditorPlugin;

fn draw_ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.label("Editor stuff...");
    });
}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                draw_ui
                    .run_in_state(AppState::Editor)
            );
    }
}
