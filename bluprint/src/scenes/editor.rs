use crate::resources::{map_file::MapFile, tabs::Tabs};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use crate::AppState;

pub struct EditorPlugin;

fn draw_ui(mut egui_context: ResMut<EguiContext>, mut tabs: ResMut<Tabs>) {
    egui::TopBottomPanel::top("toolbar").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open Map").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Bluprint Maps (*.blu)", &["blu"])
                        .set_directory(std::env::current_dir().unwrap())
                        .pick_file()
                    {
                        tabs.new_tab(MapFile::new(path));
                    }
                    ui.close_menu();
                }
                if ui.button("Close").clicked() {
                    tabs.close_current();
                    ui.close_menu();
                }
            });
        });
    });

    egui::TopBottomPanel::top("tabbar").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            if let Some(current_tab) = tabs.current_tab_idx() {
                let mut current_tab = *current_tab;
                tabs.iter().enumerate().for_each(|(index, tab)| {
                    ui.selectable_value(&mut current_tab, index, tab.file_name());
                });
                tabs.switch_to_tab(current_tab);
            }
        });
    });
}

fn load_map(
    mut commands: Commands,
) {
    commands.insert_resource(bluprint_core::example::tilemap());
    commands.insert_resource(crate::world::LoadedChunks::default());
}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tabs::default())
            .add_enter_system(AppState::Editor, load_map)
            .add_system(draw_ui.run_in_state(AppState::Editor));
    }
}
