use crate::{gui_window::GuiWindow, map_plotter::MapPlotters, robot2d::Robot2D};
use bevy::prelude::*;
use na::IsometryMatrix2;
use nalgebra as na;

#[derive(Debug, Default, Resource)]
pub struct Server {
    _window: GuiWindow,
    _plotter: MapPlotters,
    robot: Robot2D,
    pub state: bool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            _window: GuiWindow {},
            _plotter: MapPlotters {},
            robot: Robot2D::default(),
            state: true,
        }
    }

    pub fn robot_current_localization(&self) -> IsometryMatrix2<f64> {
        self.robot.current_localization()
    }
}
