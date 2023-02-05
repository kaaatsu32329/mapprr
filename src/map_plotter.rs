use bevy::prelude::*;
use egui::plot::{Line, PlotPoint, PlotPoints, Points};
use std::fs;
use yaml_rust::*;

use crate::server::Server;

#[derive(Debug, Default)]
pub struct MapPlotters {}

impl MapPlotters {
    pub fn parse_yaml_to_vec(path: &str) -> Vec<(f64, f64)> {
        let mut parsed = vec![];
        let data = fs::read_to_string(path).unwrap();
        let docs = YamlLoader::load_from_str(&data).unwrap();
        let doc = &docs[0];

        let angle_increment = doc["angle_increment"].as_f64().expect("msg");
        let range_min = doc["range_min"].as_f64().expect("msg");
        let range_max = doc["range_max"].as_f64().expect("msg");
        let ranges = doc["ranges"].as_vec().expect("msg");

        for (i, range) in ranges.iter().enumerate() {
            if let Some(val) = range.as_f64() {
                let filterd_val = Self::check_range(val, range_min, range_max);
                if let Some(v) = filterd_val {
                    parsed.push((
                        angle_increment * i as f64 * 180. / std::f64::consts::PI + 90.,
                        v,
                    ));
                }
            }
        }

        parsed
    }

    // change to PlotItem
    pub fn map_from_yaml(path: &str) -> Points {
        let point_cloud = Self::parse_yaml_to_vec(path);
        let length = point_cloud.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = point_cloud[i].1 * point_cloud[i].0.to_radians().cos();
                let y = point_cloud[i].1 * point_cloud[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Points::new(map).radius(1.5)
    }

    pub fn map_from_yaml_as_line(path: &str) -> Line {
        let point_cloud = Self::parse_yaml_to_vec(path);
        let length = point_cloud.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = point_cloud[i].1 * point_cloud[i].0.to_radians().cos();
                let y = point_cloud[i].1 * point_cloud[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Line::new(map)
    }

    pub fn parse_csv_to_vec(path: &str) -> Vec<(f64, f64)> {
        let mut parsed = Vec::new();
        let mut csv = csv::Reader::from_path(path).unwrap();
        for element in csv.records() {
            let rst = element.unwrap();
            let key = rst[0].parse::<f64>().unwrap();
            let value = rst[1].parse::<f64>().unwrap();

            parsed.push((key, value));
        }
        parsed
    }

    // change to PlotItem
    pub fn map_from_csv(path: &str) -> Points {
        let point_cloud = Self::parse_csv_to_vec(path);
        let length = point_cloud.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = point_cloud[i].1 * point_cloud[i].0.to_radians().cos();
                let y = point_cloud[i].1 * point_cloud[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Points::new(map)
    }

    pub fn map_from_csv_as_line(path: &str) -> Line {
        let point_cloud = Self::parse_csv_to_vec(path);
        let length = point_cloud.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = point_cloud[i].1 * point_cloud[i].0.to_radians().cos();
                let y = point_cloud[i].1 * point_cloud[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Line::new(map)
    }

    pub fn robot_current_localization(server: ResMut<Server>) -> Points {
        let current = server.robot_current_localization();

        let points = PlotPoints::Owned(vec![PlotPoint {
            x: current.0,
            y: current.1,
        }]);

        Points::new(points).radius(5.0)
    }

    fn check_range(val: f64, min: f64, max: f64) -> Option<f64> {
        if val > min && val < max {
            Some(val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_yaml_to_vec() {
        let path = "sample/ros2_scan_sample.yaml";
        let parsed = MapPlotters::parse_yaml_to_vec(path);
        println!("{:?}", parsed);
    }
}
