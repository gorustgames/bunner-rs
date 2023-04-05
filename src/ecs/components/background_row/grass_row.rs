use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::{get_road_or_water_row, RowType};
use std::any::Any;

#[derive(Debug)]
pub struct GrassRow {
    index: i8,
    mask: Option<[bool; 12]>,

    /// relevant only if mask is Some(...)
    /// in that case this attribute represents whether
    /// mask is top bushes row or bottom bushes row
    row_with_top_bushes: bool,
}

impl GrassRow {
    pub fn new_grass_row(index: i8) -> Self {
        GrassRow {
            index,
            mask: None,
            row_with_top_bushes: false,
        }
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
            row_with_top_bushes: self.row_with_top_bushes,
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

    fn set_row_data(&mut self, data: Box<dyn Any>) {
        if let Ok(row_with_top_bushes) = data.downcast::<bool>() {
            if *row_with_top_bushes && self.mask.is_some() {
                self.row_with_top_bushes = *row_with_top_bushes;
            }
        }
    }

    fn get_row_data(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.row_with_top_bushes))
    }
}
