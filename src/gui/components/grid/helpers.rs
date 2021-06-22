use log::info;

use std::ops::RangeInclusive;

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
        let first = (self.y / self.grid_dist).floor() as isize;
        let visible = (self.h / self.grid_dist).ceil() as isize;

        first..=first + visible
    }

    pub fn horiz_lines(&self) -> RangeInclusive<isize> {
        let first = (self.x / self.grid_dist).floor() as isize;
        let visible = (self.w / self.grid_dist).ceil() as isize;

        first..=first + visible
    }
}

