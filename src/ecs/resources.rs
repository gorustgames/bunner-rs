use crate::ecs::components::background_row::BackgroundRow;
use sliding_window::typenum::consts::*;
use sliding_window::*;

pub struct BackgroundRows {
    rows: SlidingWindow<BackgroundRow, U20>,
}

impl BackgroundRows {
    pub fn new() -> Self {
        BackgroundRows {
            rows: SlidingWindow::new(),
        }
    }

    pub fn add_row(&mut self, bg_row: BackgroundRow) {
        self.rows.insert(bg_row);
    }
}
