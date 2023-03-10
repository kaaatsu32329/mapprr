use egui::plot::{Line, PlotPoint, PlotPoints, Points};
use std::fs;
use yaml_rust::*;

use crate::server::{convert_file_name_to_map_data_type, MapDataType, SampleType, Server};

#[derive(Debug, Default)]
pub struct MapPlotters {
    plot_buffer: Vec<(f64, f64)>,
}

impl MapPlotters {
    pub fn parse_to_vec(&mut self, path: &str) -> Result<(), std::string::String> {
        self.plot_buffer.clear();

        let data_type = convert_file_name_to_map_data_type(path);

        if let Some(data) = data_type {
            match data {
                MapDataType::Sample(sample) => match sample {
                    SampleType::Yaml => {
                        let data = fs::read_to_string(path).unwrap();
                        let docs = YamlLoader::load_from_str(&data).unwrap();
                        let doc = &docs[0];

                        let angle_increment = doc["angle_increment"].as_f64().expect("msg");
                        let range_min = doc["range_min"].as_f64().expect("msg");
                        let range_max = doc["range_max"].as_f64().expect("msg");
                        let ranges = doc["ranges"].as_vec().expect("msg");

                        for (i, range) in ranges.iter().enumerate() {
                            if let Some(val) = range.as_f64() {
                                let filterd_val = check_range(val, range_min, range_max);
                                if let Some(v) = filterd_val {
                                    self.plot_buffer.push((
                                        angle_increment * i as f64 * 180. / std::f64::consts::PI
                                            + 90.,
                                        v,
                                    ));
                                }
                            }
                        }
                    }
                    SampleType::Csv => {
                        let mut csv = csv::Reader::from_path(path).unwrap();
                        for element in csv.records() {
                            let rst = element.unwrap();
                            let key = rst[0].parse::<f64>().unwrap();
                            let value = rst[1].parse::<f64>().unwrap();

                            self.plot_buffer.push((key, value));
                        }
                    }
                },
                MapDataType::Dummy => todo!(),
                MapDataType::Ros => todo!(),
                MapDataType::Ros2 => todo!(),
            }
        }
        Ok(())
    }

    // change to PlotItem
    pub fn map_from_data(&mut self, path: &str) -> Points {
        self.parse_to_vec(path).unwrap();
        let length = self.plot_buffer.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = self.plot_buffer[i].1 * self.plot_buffer[i].0.to_radians().cos();
                let y = self.plot_buffer[i].1 * self.plot_buffer[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Points::new(map).radius(1.5)
    }

    pub fn map_as_line(&mut self, path: &str) -> Line {
        self.parse_to_vec(path).unwrap();
        let length = self.plot_buffer.len();
        let map: PlotPoints = (0..length)
            .map(|i| {
                let x = self.plot_buffer[i].1 * self.plot_buffer[i].0.to_radians().cos();
                let y = self.plot_buffer[i].1 * self.plot_buffer[i].0.to_radians().sin();
                [x, y]
            })
            .collect();

        Line::new(map)
    }

    pub fn robot_current_localization(&self, server: &Server) -> Points {
        let current = server.robot_current_localization();

        let points = PlotPoints::Owned(vec![PlotPoint {
            x: current.translation.x,
            y: current.translation.y,
        }]);

        Points::new(points).radius(5.0)
    }
}

fn check_range(val: f64, min: f64, max: f64) -> Option<f64> {
    if val > min && val < max {
        Some(val)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_yaml_to_vec() {
        let path = "sample/ros2_scan_sample.yaml";
        let mut plotter = MapPlotters::default();
        plotter.parse_to_vec(path).unwrap();
        println!("{:?}", plotter.plot_buffer);
    }
}
