use crate::ecs::components::background_row::road_row::RoadRow;
use crate::ecs::components::background_row::row::Row;

#[derive(Debug)]
pub struct PavementRow {
    index: i8,
}

impl PavementRow {
    pub fn new_pavement_row(index: i8) -> Self {
        PavementRow { index }
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
    fn get_index(&self) -> i8 {
        self.index
    }
    fn get_img_base(&self) -> String {
        "side".to_string()
    }
}
