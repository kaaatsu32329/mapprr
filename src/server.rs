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
    map_data: MapDataType,
    robot: Robot2D,
    pub state: bool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            plotter: MapPlotters::default(),
            map_data: MapDataType::default(),
            robot: Robot2D::default(),
            state: true,
        }
    }

    pub fn get_map_point(&mut self) -> Points {
        match &self.map_data {
            MapDataType::Sample(sample_type) => match sample_type {
                SampleType::Yaml => self.plotter.map_from_data(SAMPLE_YAML_FILE_PATH),
                SampleType::Csv => self.plotter.map_from_data(SAMPLE_CSV_FILE_PATH),
            },
            MapDataType::Dummy => todo!(),
            MapDataType::Ros => todo!(),
            MapDataType::Ros2 => todo!(),
        }
    }

    pub fn robot_current_localization(&self) -> IsometryMatrix2<f64> {
        self.robot.current_localization()
    }

    pub fn robot_current_pose_point(&self) -> Points {
        self.plotter.robot_current_localization(self)
    }
}

pub(crate) fn convert_file_name_to_map_data_type(path: &str) -> Option<MapDataType> {
    let binding = std::path::PathBuf::from(path);
    let extension = binding.extension();
    if let Some(ext_as_os_str) = extension {
        let ext_as_str = ext_as_os_str.to_str();
        if let Some(ext) = ext_as_str {
            match ext {
                "yaml" => return Some(MapDataType::Sample(SampleType::Yaml)),
                "csv" => return Some(MapDataType::Sample(SampleType::Csv)),
                _ => return None,
            };
        }
    }
    None
}

#[derive(Debug)]
pub(crate) enum MapDataType {
    Sample(SampleType),
    Dummy,
    // TODO
    Ros,
    // TODO
    Ros2,
}

#[derive(Debug, Default)]
pub(crate) enum SampleType {
    #[default]
    Yaml,
    Csv,
}

impl Default for MapDataType {
    fn default() -> Self {
        MapDataType::Sample(SampleType::default())
    }
}
