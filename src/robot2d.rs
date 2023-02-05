use na::IsometryMatrix2;
use nalgebra as na;

#[derive(Debug, Default)]
pub(crate) struct Robot2D {
    pose: na::IsometryMatrix2<f64>,
}

impl Robot2D {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self {
            pose: na::IsometryMatrix2::new(na::Vector2::new(x, y), theta),
        }
    }

    // TODO: Change return type to nalgebra isometry.
    pub fn current_localization(&self) -> IsometryMatrix2<f64> {
        self.pose
    }

    pub fn update_localization(&mut self, diff: IsometryMatrix2<f64>) {
        self.pose.translation.x += diff.translation.x;
        self.pose.translation.y += diff.translation.y;
        self.pose.rotation *= diff.rotation;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_robot2d() {
        let mut robot = Robot2D::new(1.0, 2.0, 3.0);
        let diff = na::IsometryMatrix2::new(na::Vector2::new(4.0, 5.0), 6.0);

        robot.update_localization(diff);

        assert_approx_eq!(robot.pose.translation.x, 5.0);
        assert_approx_eq!(robot.pose.translation.y, 7.0);
        assert_approx_eq!(
            robot.pose.rotation.angle(),
            9.0 % (2. * std::f64::consts::PI)
        );
    }
}
