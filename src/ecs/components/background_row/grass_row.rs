use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::{get_road_or_water_row, RowType};

#[derive(Debug)]
pub struct GrassRow {
    index: i8,
    mask: Option<[bool; 12]>,
}

impl GrassRow {
    pub fn new_grass_row(index: i8) -> Self {
        GrassRow { index, mask: None }
    }
}

impl Row for GrassRow {
    fn next(&self) -> Box<dyn Row> {
        match self.index {
            // match inclusive range
            0..=5 => Box::new(GrassRow::new_grass_row(self.index + 8)),
            6 => Box::new(GrassRow::new_grass_row(7)),
            7 => Box::new(GrassRow::new_grass_row(15)),
            8..=14 => Box::new(GrassRow::new_grass_row(self.index + 1)),
            _ => get_road_or_water_row(),
        }
    }

    fn clone_row(&self) -> Box<dyn Row> {
        Box::new(Self {
            index: self.index,
            mask: self.mask,
        })
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "grass".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::GRASS
    }

    fn get_row_mask(&self) -> Option<[bool; 12]> {
        self.mask
    }

    fn set_row_mask(&mut self, mask: [bool; 12]) {
        self.mask = Some(mask);
    }
}
