use crate::ecs::components::background_row::road_row::RoadRow;
use crate::ecs::components::background_row::row::Row;
use crate::ecs::components::background_row::RowType;
use crate::get_uuid;
use std::any::Any;

#[derive(Debug)]
pub struct PavementRow {
    index: i8,
    uuid: String,
    y: f32,
}

impl PavementRow {
    pub fn new_pavement_row(index: i8) -> Self {
        PavementRow {
            index,
            uuid: get_uuid(),
            y: 0.,
        }
    }
}

impl Row for PavementRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index < 2 {
            Box::new(PavementRow::new_pavement_row(self.index + 1))
        } else {
            Box::new(RoadRow::new_road_row(0))
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
        "side".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::PAVEMENT
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
