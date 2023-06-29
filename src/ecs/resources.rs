use crate::ecs::components::background_row::{Row, RowType};
use crate::ecs::components::MovementDirection;
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

    pub fn get_row(&self, row_index: usize) -> Option<Box<dyn Row>> {
        return if self.rows.count() == 0 || row_index > self.rows.count() - 1 {
            None
        } else {
            Some(self.rows[row_index].clone_row())
        };
    }

    pub fn debug_print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row);
        }
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
    WaterLog(MovementDirection),

    RailsOnly,

    RailsTrain,

    RoadOnly,

    RoadCar,

    /// rows like Dirt (i.e. with no child objects)
    /// fall under category Other, here no action is needed
    Other,
}

/// this enums determines in which direction player movement is blocked
/// by bushes. since bushes are always rectangular with no corners single
/// value is always enough, it is not needed to block movement in two directions
/// at the same time.
#[derive(Debug, PartialEq)]
pub enum PlayerMovementBlockedDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

/// Determines what type of spot player is standing on
#[derive(Debug)]
pub struct PlayerPosition {
    /// determines active row
    pub row_type: RowType,

    /// which row player is standing on
    /// very bottom row has index 0
    /// we have together 800 / 40 = 20 rows
    pub row_index: i8,

    /// which column within row player is standing on
    /// very left columns has index 0
    /// we have 480 px / 40 px = 12 columns
    pub col_index: i8,

    /// within active row determines whether player
    /// is standing on row only (e.g.g RoadOnly) or is colliding with
    /// some child object (i.e. RoadCar)
    pub collision_type: CollisionType,

    /// global x coordinate of player
    pub player_x: f32,

    /// global y coordinate of player
    pub player_y: f32,

    /// determines if player movement is blocked in any way
    pub movement_blocked_dir: PlayerMovementBlockedDirection,
}

impl PlayerPosition {
    /// serves for setting initial position when game starts
    /// player stands on grass
    pub fn new() -> Self {
        PlayerPosition {
            row_type: RowType::GRASS,
            row_index: 7, // initially the player is at row 8
            col_index: 5, // initially the player is at column 6
            collision_type: CollisionType::Other,
            player_x: 0., // doesn't really matter what we put here, it will be immediately updated by respective ecs system
            player_y: 0.,
            movement_blocked_dir: PlayerMovementBlockedDirection::None,
        }
    }

    pub fn set_water_ok(&mut self, direction: MovementDirection) {
        self.row_type = RowType::WATER;
        self.collision_type = CollisionType::WaterLog(direction);
    }

    pub fn set_water_ko(&mut self) {
        self.row_type = RowType::WATER;
        self.collision_type = CollisionType::WaterOnly;
    }

    pub fn set_road_ok(&mut self) {
        self.row_type = RowType::ROAD;
        self.collision_type = CollisionType::RoadOnly;
    }

    pub fn set_road_ko(&mut self) {
        self.row_type = RowType::ROAD;
        self.collision_type = CollisionType::RoadCar;
    }

    pub fn set_rail_ok(&mut self) {
        self.row_type = RowType::RAIL;
        self.collision_type = CollisionType::RailsOnly;
    }

    pub fn set_rail_ko(&mut self) {
        self.row_type = RowType::RAIL;
        self.collision_type = CollisionType::RailsTrain;
    }

    pub fn set_dirt(&mut self) {
        self.row_type = RowType::DIRT;
        self.collision_type = CollisionType::Other;
    }

    pub fn set_grass(&mut self) {
        self.row_type = RowType::GRASS;
        self.collision_type = CollisionType::Other;
    }

    pub fn set_pavement(&mut self) {
        self.row_type = RowType::PAVEMENT;
        self.collision_type = CollisionType::Other;
    }
}

pub struct BackgroundScrollingEnabled {
    pub enabled: bool,
}

impl BackgroundScrollingEnabled {
    pub fn new() -> Self {
        BackgroundScrollingEnabled { enabled: true }
    }
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
