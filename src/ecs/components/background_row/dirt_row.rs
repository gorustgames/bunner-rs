use crate::ecs::components::background_row::get_road_or_water_row;
use crate::ecs::components::background_row::row::Row;

#[derive(Debug)]
pub struct DirtRow {
    index: i8,
}

impl DirtRow {
    pub fn new_dirt_row(index: i8) -> Self {
        DirtRow { index }
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
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "dirt".to_string()
    }
}
