#[derive(Debug, Default)]
pub(crate) struct Robot2D {
    x: f64,
    y: f64,
    theta: f64,
}

impl Robot2D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }

    // TODO: Change return type to nalgebra isometry.
    pub fn current_localization(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.theta)
    }
}
