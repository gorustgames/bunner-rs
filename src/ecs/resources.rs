use crate::ecs::components::background_row::Row;
use sliding_window::typenum::consts::*;
use sliding_window::*;

pub struct BackgroundRows {
    rows: SlidingWindow<Box<dyn Row>, U20>,
}

impl BackgroundRows {
    pub fn new() -> Self {
        BackgroundRows {
            rows: SlidingWindow::new(),
        }
    }

    pub fn add_row(&mut self, row: Box<dyn Row>) {
        self.rows.insert(row);
    }
}
