use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::{get_road_or_water_row, RowType};
use crate::get_uuid;
use std::any::Any;

#[derive(Debug)]
pub struct GrassRow {
    index: i8,
    uuid: String,
    mask: Option<[bool; 12]>,

    /// relevant only if mask is Some(...)
    /// in that case this attribute represents whether
    /// mask is top bushes row or bottom bushes row
    row_with_top_bushes: bool,
    y: f32,
}

impl GrassRow {
    pub fn new_grass_row(index: i8) -> Self {
        GrassRow {
            index,

            uuid: get_uuid(),
            mask: None,
            row_with_top_bushes: false,
            y: 0.,
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
            uuid: self.uuid.to_owned(),
            mask: self.mask,
            row_with_top_bushes: self.row_with_top_bushes,
            y: self.y,
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

    /// row mask is array of booleans that can represent some specific data for given row
    /// for grass meaning of mask is as follows:
    /// false - there is hedge/bush present on given 40px part of the row
    /// true - there is a gap on respective 40px row part
    fn set_row_mask(&mut self, mask: [bool; 12]) {
        self.mask = Some(mask);
    }

    /// row data represents arbitrary data associated with respective row
    /// for grass row meaning is following:
    /// true - this is the row with bushes and it is TOP row
    /// false- this is the row with bushes and it is BOTTOM row
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
