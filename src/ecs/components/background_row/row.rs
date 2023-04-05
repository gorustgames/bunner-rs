use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RowType {
    DIRT,
    GRASS,
    PAVEMENT,
    RAIL,
    ROAD,
    WATER,
}

pub trait Row: Send + Sync + Debug {
    fn next(&self) -> Box<dyn Row>;

    fn clone_row(&self) -> Box<dyn Row>;

    fn get_index(&self) -> i8;

    fn get_img_base(&self) -> String;

    fn get_img_name(&self) -> String {
        format!("images/{}{}.png", self.get_img_base(), self.get_index())
    }

    fn get_row_type(&self) -> RowType;

    /// mask is currently used only by grass rows to
    /// represent hedge/bushes
    fn get_row_mask(&self) -> Option<[bool; 12]>;

    fn set_row_mask(&mut self, mask: [bool; 12]);

    /// set row arbitrary data (any)
    fn set_row_data(&mut self, data: Box<dyn Any>);

    fn get_row_data(&self) -> Option<Box<dyn Any>>;
}
