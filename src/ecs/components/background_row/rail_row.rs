use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::{get_road_or_water_row, RowType};

#[derive(Debug)]
pub struct RailRow {
    index: i8,
}

impl RailRow {
    pub fn new_rail_row(index: i8) -> Self {
        RailRow { index }
    }
}

impl Row for RailRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 3 {
            Box::new(RailRow::new_rail_row(self.index + 1))
        } else {
            get_road_or_water_row()
        }
    }

    fn clone_row(&self) -> Box<dyn Row> {
        Box::new(Self { index: self.index })
    }

    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "rail".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::RAIL
    }

    fn get_row_mask(&self) -> Option<[bool; 12]> {
        None
    }

    fn set_row_mask(&mut self, _: [bool; 12]) {
        return;
    }
}
