use crate::ecs::components::background_row::dirt_row::DirtRow;
use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::RowType;
use crate::{get_random_float, get_random_i8};
use std::any::Any;

#[derive(Debug)]
pub struct WaterRow {
    index: i8,
}

impl WaterRow {
    pub fn new_water_row(index: i8) -> Self {
        WaterRow { index }
    }
}

impl Row for WaterRow {
    fn next(&self) -> Box<dyn Row> {
        // After 2 water rows, there's a 50-50 chance of the next row being either another water row, or a dirt row
        if (self.index == 7) || (self.index >= 1 && get_random_float() < 0.5) {
            Box::new(DirtRow::new_dirt_row(get_random_i8(4, 6)))
        } else {
            Box::new(WaterRow::new_water_row(self.index + 1))
        }
    }

    fn clone_row(&self) -> Box<dyn Row> {
        Box::new(Self { index: self.index })
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "water".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::WATER
    }

    fn get_row_mask(&self) -> Option<[bool; 12]> {
        None
    }

    fn set_row_mask(&mut self, _: [bool; 12]) {
        return;
    }

    fn set_row_data(&mut self, _: Box<dyn Any>) {
        return;
    }

    fn get_row_data(&self) -> Option<Box<dyn Any>> {
        None
    }
}
