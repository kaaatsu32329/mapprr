use std::ops::RangeInclusive;

use crate::{map_plotter::MapPlotters, server::Server};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::plot::Plot;

pub struct MapprrApp {}

impl MapprrApp {
    pub fn run() {
        App::new()
            .insert_resource(Server::new())
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_system(Self::ui_example_system)
            .run();
    }

    pub fn laserscan() {}

    fn ui_example_system(
        key_board_input: Res<Input<KeyCode>>,
        mut egui_context: ResMut<EguiContext>,
        mut server: ResMut<Server>,
    ) {
        if key_board_input.just_pressed(KeyCode::Left)
            || key_board_input.just_pressed(KeyCode::Right)
        {
            server.state = !server.state;
        }

        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            Plot::new("cosine").data_aspect(1.).show(ui, |plot_ui| {
                // plot_ui.points(Self::obstacle_plot_points(server.state))
                // plot_ui.points(MapPlotters::map_from_csv("sample/circle_sample.csv"));
                plot_ui.points(MapPlotters::map_from_yaml("sample/ros2_scan_sample.yaml"));
                // plot_ui.line(Self::map_from_csv_as_line())
                plot_ui.points(MapPlotters::robot_current_localization(server))
            });
            ui.label("sin");
        });

        egui::SidePanel::new(egui::panel::Side::Left, "Controller")
            .width_range(RangeInclusive::new(100f32, 200f32))
            .show(egui_context.ctx_mut(), |_ui| {
                //     ui.button("Start");
            });

        egui::SidePanel::new(egui::panel::Side::Right, "Monitor")
            .width_range(RangeInclusive::new(100f32, 200f32))
            .show(egui_context.ctx_mut(), |_ui| {});

        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "Debug")
            .show(egui_context.ctx_mut(), |_ui| {});

        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "Header")
            .show(egui_context.ctx_mut(), |_ui| {});
    }
}
