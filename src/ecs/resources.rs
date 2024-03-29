use crate::ecs::components::background_row::{Row, RowType};
use crate::ecs::components::MovementDirection;
use crate::SEGMENT_HEIGHT;
use bevy::prelude::*;

/// we have 21 rows in sliding window even though screen has in theory only 20 rows
/// i..e 20 x 40 px = 800 px in total. This is so that very bottom row which is already not
/// visible in total (because it has partly scrolled off) is still available in data

const SLIDING_WINDOW_SIZE: usize = 21;

pub struct BackgroundRows {
    data: Vec<Box<dyn Row>>,
    window_size: usize,
}

impl BackgroundRows {
    pub fn new() -> Self {
        BackgroundRows {
            data: Vec::with_capacity(SLIDING_WINDOW_SIZE),
            window_size: SLIDING_WINDOW_SIZE,
        }
    }

    pub fn add_row(&mut self, row: Box<dyn Row>) {
        if self.data.len() >= self.window_size {
            self.data.remove(0);
        }
        self.data.push(row);
    }

    pub fn last_row(&self) -> Option<Box<dyn Row>> {
        return if self.data.len() == 0 {
            None
        } else {
            Some(self.data[self.data.len() - 1].clone_row())
        };
    }

    pub fn get_row(&self, row_index: usize) -> Option<Box<dyn Row>> {
        return if self.data.len() == 0 || row_index > self.data.len() - 1 {
            None
        } else {
            Some(self.data[row_index].clone_row())
        };
    }

    pub fn debug_print(&self) {
        for n in (0..self.data.len()).rev() {
            println!("row {:?} {:?}", n, &self.data[n]);
        }
    }

    pub fn set_row_y_by_row_uuid(&mut self, row_uuid: &str, y: f32) {
        for n in 0..self.data.len() {
            if self.data[n].get_row_uuid() == row_uuid {
                self.data[n].set_row_y(y);
                break;
            }
        }
    }

    pub fn get_player_row(&self, player_y: f32) -> Option<i8> {
        for n in 0..self.data.len() {
            if player_y + 20. >= self.data[n].get_row_y()
                && player_y + 20. < self.data[n].get_row_y() + SEGMENT_HEIGHT
            {
                return Some(n as i8);
            }
        }
        return None; // this is basically error, we are unable to determine player row
    }

    /// replacement for fn player_row_to_coords
    pub fn get_player_row_to_coords(&self, player_row: i8) -> (f32, f32) {
        (
            self.data[player_row as usize].get_row_y(),
            self.data[player_row as usize].get_row_y() + 40.,
        )
    }
}

pub struct MenuData {
    pub button_entity_exit: Entity,
    pub button_entity_start: Entity,
}

#[derive(Debug, PartialEq)]
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

    /// time from last change
    /// used to prevent debouncing
    pub changed: u64,
}

impl BackgroundScrollingEnabled {
    pub fn new() -> Self {
        BackgroundScrollingEnabled {
            enabled: true,
            changed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::components::background_row::{GrassRow, PavementRow};

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

    // cargo test test_custom_sliding_window -- --show-output
    #[test]
    fn test_custom_sliding_window() {
        let mut sliding_window = BackgroundRows::new();

        for i in 1..22 {
            sliding_window.add_row(Box::new(GrassRow::new_grass_row(i)));
        }

        let oldest_minus_one_row_before_slide = sliding_window.get_row(1);

        sliding_window.debug_print();
        sliding_window.add_row(Box::new(PavementRow::new_pavement_row(5)));
        println!("--------");
        let oldest_row_after_slide = sliding_window.get_row(0);
        sliding_window.debug_print();

        assert_eq!(
            oldest_minus_one_row_before_slide.unwrap().get_row_uuid(),
            oldest_row_after_slide.unwrap().get_row_uuid()
        );
    }
}
