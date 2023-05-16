use crate::ecs::components::background_row::{Row, RowType};
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

#[derive(Debug)]
pub enum CollisionType {
    /// player is standing on water row
    /// and on no log -> ko!
    WaterOnly,

    /// player is standing on water row
    /// and on the log -> ok!
    WaterLog,
    RailsOnly,
    RailsTrain,
    RoadOnly,
    RoadCar,

    /// rows like Dirt (i.e. with no child objects)
    /// fall under category Other, here no action is needed
    Other,
}

/// Determines what type of spot player is standing on
pub struct PlayerPosition {
    /// determines active row
    pub row_type: RowType,

    /// within active row determines whether player
    /// is standing on row only (e.g.g RoadOnly) or is colliding with
    /// some child object (i.e. RoadCar)
    pub collision_type: CollisionType,
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
