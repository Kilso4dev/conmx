use iced::{
    canvas,
    Rectangle,
};

pub trait Drawable {
    fn draw(&self, frame: &mut canvas::Frame);
    fn get_bounding_box(&self) -> Rectangle;
}
