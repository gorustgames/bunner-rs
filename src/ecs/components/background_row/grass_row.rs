use crate::ecs::components::background_row::get_road_or_water_row;
use crate::ecs::components::background_row::row::Row;

#[derive(Debug)]
pub struct GrassRow {
    index: i8,
}

impl GrassRow {
    pub fn new_grass_row(index: i8) -> Self {
        GrassRow { index }
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
        Box::new(Self { index: self.index })
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "grass".to_string()
    }
}
