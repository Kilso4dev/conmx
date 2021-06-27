use log::info;

use std::ops::RangeInclusive;

use iced::{
    Rectangle,
    Point,
    canvas,
};

#[derive(Debug, Clone)]
pub struct Region {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub grid_dist: f32,
}

impl Region {
    pub fn vert_lines(&self) -> RangeInclusive<isize> {
        let first = (self.x / self.grid_dist).floor() as isize;
        let last = ((self.x+self.w) / self.grid_dist).ceil() as isize;

        first..=last
    }

    pub fn horiz_lines(&self) -> RangeInclusive<isize> {
        let first = (self.y / self.grid_dist).floor() as isize;
        let last = ((self.y+self.h) / self.grid_dist).ceil() as isize;

        first..=last
    }
}

pub fn rounded_rect_path(dim: Rectangle, radius: f32) -> canvas::Path {
    let mut path_builder = canvas::path::Builder::new();

    path_builder.move_to(Point::new(radius, 0.));

    // Upper left
    path_builder.arc_to(
        Point::new(radius, 0.),
        Point::new(0., radius),
        radius,
        );
    path_builder.line_to(Point::new(0., dim.height-radius));

    // Lower left
    path_builder.arc_to(
        Point::new(0., dim.height-radius),
        Point::new(radius, dim.height),
        radius
        );
    path_builder.line_to(Point::new(dim.width-radius, dim.height));

    // Lower right
    path_builder.arc_to(
        Point::new(dim.width-radius, dim.height),
        Point::new(dim.width, dim.height-radius),
        radius
        );
    path_builder.line_to(Point::new(dim.width, radius));
    
    // Upper right
    path_builder.arc_to(
        Point::new(dim.width, radius),
        Point::new(dim.width-radius, 0.),
        radius
        );
    path_builder.close();

    path_builder.build()
}
