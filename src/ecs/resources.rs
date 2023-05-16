use crate::ecs::components::background_row::Row;
use bevy::prelude::*;
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

    pub fn last_row(&self) -> Option<Box<dyn Row>> {
        return if self.rows.count() == 0 {
            None
        } else {
            Some(self.rows[self.rows.count() - 1].clone_row())
        };
    }
}

pub struct MenuData {
    pub button_entity_exit: Entity,
    pub button_entity_start: Entity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::components::background_row::GrassRow;

    #[test]
    fn test_last_row() {
        let mut bgr = BackgroundRows::new();
        assert_eq!(bgr.last_row().is_none(), true);
        let mut grass_row = GrassRow::new_grass_row(0);
        let mask: [bool; 12] = [
            true, true, true, false, false, false, true, true, true, false, true, false,
        ];
        grass_row.set_row_mask(mask);
        bgr.add_row(Box::new(grass_row));
        let previous_row = bgr.last_row();
        assert_eq!(previous_row.unwrap().get_row_mask(), Some(mask));
    }
}
