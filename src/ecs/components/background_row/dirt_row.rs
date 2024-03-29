use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::{get_road_or_water_row, RowType};
use crate::get_uuid;
use std::any::Any;

#[derive(Debug)]
pub struct DirtRow {
    index: i8,
    uuid: String,
    y: f32,
}

impl DirtRow {
    pub fn new_dirt_row(index: i8) -> Self {
        DirtRow {
            index,
            uuid: get_uuid(),
            y: 0.,
        }
    }
}

impl Row for DirtRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            0..=5 => Box::new(DirtRow::new_dirt_row(self.index + 8)),
            6 => Box::new(DirtRow::new_dirt_row(7)),
            7 => Box::new(DirtRow::new_dirt_row(15)),
            8..=14 => Box::new(DirtRow::new_dirt_row(self.index + 1)),
            _ => get_road_or_water_row(),
        }
    }

    fn clone_row(&self) -> Box<dyn Row> {
        Box::new(Self {
            index: self.index,
            uuid: self.uuid.to_owned(),
            y: self.y,
        })
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "dirt".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::DIRT
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

    fn get_row_uuid(&self) -> String {
        self.uuid.to_owned()
    }

    fn get_row_y(&self) -> f32 {
        self.y
    }

    fn set_row_y(&mut self, y: f32) {
        self.y = y;
    }
}
