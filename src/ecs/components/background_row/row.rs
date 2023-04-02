use std::fmt::Debug;

pub trait Row: Send + Sync + Debug {
    fn next(&self) -> Box<dyn Row>;

    fn clone_row(&self) -> Box<dyn Row>;

    fn get_index(&self) -> i8;

    fn get_img_base(&self) -> String;

    fn get_img_name(&self) -> String {
        format!("images/{}{}.png", self.get_img_base(), self.get_index())
    }
}
