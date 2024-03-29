use crate::ecs::components::background_row::grass_row::*;
use crate::ecs::components::background_row::pavement_row::*;
use crate::ecs::components::background_row::rail_row::*;
use crate::ecs::components::background_row::row::*;
use crate::{get_random_float, get_random_i8, get_uuid};
use std::any::Any;

#[derive(Debug)]
pub struct RoadRow {
    index: i8,
    uuid: String,
    y: f32,
}

impl RoadRow {
    pub fn new_road_row(index: i8) -> Self {
        RoadRow {
            index,
            uuid: get_uuid(),
            y: 0.,
        }
    }
}

impl Row for RoadRow {
    fn next(&self) -> Box<dyn Row> {
        if self.index == 0 {
            Box::new(RoadRow::new_road_row(self.index + 1))
        } else if self.index < 5 {
            let r = get_random_float();
            if r < 0.8 {
                Box::new(RoadRow::new_road_row(self.index + 1))
            } else if r < 0.88 {
                Box::new(GrassRow::new_grass_row(get_random_i8(0, 6)))
            } else if r < 0.94 {
                Box::new(RailRow::new_rail_row(0))
            } else {
                Box::new(PavementRow::new_pavement_row(0))
            }
        } else {
            let r = get_random_float();
            if r < 0.6 {
                Box::new(GrassRow::new_grass_row(get_random_i8(0, 6)))
            } else if r < 0.9 {
                Box::new(RailRow::new_rail_row(0))
            } else {
                Box::new(PavementRow::new_pavement_row(0))
            }
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
        "road".to_string()
    }

    fn get_row_type(&self) -> RowType {
        RowType::ROAD
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
