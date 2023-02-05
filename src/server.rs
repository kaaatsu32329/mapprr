use crate::{map_plotter::MapPlotters, robot2d::Robot2D};
use bevy::prelude::*;
use egui::plot::Points;
use na::IsometryMatrix2;
use nalgebra as na;

const SAMPLE_YAML_FILE_PATH: &str = "sample/ros2_scan_sample.yaml";
const SAMPLE_CSV_FILE_PATH: &str = "sample/circle_sample.csv";

#[derive(Debug, Default, Resource)]
pub struct Server {
    plotter: MapPlotters,
    map_data: MapData,
    robot: Robot2D,
    pub state: bool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            plotter: MapPlotters::default(),
            map_data: MapData::default(),
            robot: Robot2D::default(),
            state: true,
        }
    }

    pub fn get_map_point(&self) -> Points {
        match &self.map_data {
            MapData::Sample(sample_type) => match sample_type {
                SampleType::Yaml => self.plotter.map_from_yaml(SAMPLE_YAML_FILE_PATH),
                SampleType::Csv => self.plotter.map_from_csv(SAMPLE_CSV_FILE_PATH),
            },
            MapData::Dummy => todo!(),
            MapData::Ros => todo!(),
            MapData::Ros2 => todo!(),
        }
    }

    pub fn robot_current_localization(&self) -> IsometryMatrix2<f64> {
        self.robot.current_localization()
    }

    pub fn robot_current_pose_point(&self) -> Points {
        self.plotter.robot_current_localization(self)
    }
}

#[derive(Debug)]
enum MapData {
    Sample(SampleType),
    Dummy,
    // TODO
    Ros,
    // TODO
    Ros2,
}

#[derive(Debug, Default)]
enum SampleType {
    #[default]
    Yaml,
    Csv,
}

impl Default for MapData {
    fn default() -> Self {
        MapData::Sample(SampleType::default())
    }
}
