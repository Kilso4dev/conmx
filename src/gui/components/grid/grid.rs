use iced::{
    Length,
    canvas,
    Element,
    Point,
    Color,
    Size,
};

use log::info;

use iced::Vector;

use super::helpers;

pub enum Msg {
    GridChanged(f32),
}

#[derive(Debug)]
pub struct Grid {
    translation: Vector,
    scaling: f32,
    grid_dist: f32,

    grid_cache: canvas::Cache,
    background_cache: canvas::Cache,
}

impl Grid {
    pub fn new() -> Self{
        Self {
            translation: Vector::new(0., 0.),
            scaling: 20.,
            grid_dist: 1.,

            grid_cache: canvas::Cache::new(),
            background_cache: canvas::Cache::new(),
        }
    }

    pub fn view<'a>(&'a mut self) -> Element<'a, Msg> {
        canvas::Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn calc_visible_region(&self, size: Size, grid_dist: f32) -> helpers::Region {
        let w = size.width / self.scaling;
        let h = size.height / self.scaling;

        helpers::Region {
            x: -self.translation.x - w/2.,
            y: -self.translation.y - h/2.,
            w, h,
            grid_dist,
        }
    }

    fn project_point(&self, pos: Point, size: Size) -> Point {
        let reg = self.calc_visible_region(size, self.grid_dist);

        Point::new(
            pos.x / self.scaling + reg.x,
            pos.y / self.scaling + reg.y,
        )
    }
}

impl<'a> canvas::Program<Msg> for Grid {
    fn update(
        &mut self,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> ( canvas::event::Status, Option<Msg> ) {
        //info!("Event: {:?}, bounds: {:?}, cursor: {:?}", event, bounds, cursor);
        (canvas::event::Status::Captured, None)
    }

    fn draw(
        &self,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let bg = self.background_cache.draw(bounds.size(), |frame| {
            let bg = canvas::Path::rectangle(iced::Point::ORIGIN, frame.size());
            frame.fill(&bg, Color::from_rgb8(0x33, 0x33, 0x33));

            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);

                let region = self.calc_visible_region(frame.size(), self.grid_dist);

            });
        });

        let grid = self.grid_cache.draw(bounds.size(), |frame| {
            let line_width = 0.3;

            frame.translate(center);
            frame.scale(self.scaling);
            frame.translate(self.translation);

            let region = self.calc_visible_region(frame.size(), self.grid_dist);
            let verts = region.vert_lines();
            let horizs = region.horiz_lines();
            let color = Color::from_rgb8(70, 74, 83);

            //info!("region: {:?} \nverts: {:?} horizs: {:?}", region, verts, horizs);
            info!("Frame size: {:?}", frame.size());

            let circle_path = canvas::Path::circle(Point::new(0., 0.), line_width);

            let stroke_style = canvas::Stroke::default()
                .with_color(Color::from_rgb8(132, 59, 235))
                .with_width(line_width);

            frame.stroke(&circle_path, stroke_style.clone());

            for ch in horizs.clone() {
                let c_line_stroke = canvas::Path::line(
                    Point::new(*verts.start() as f32        , ch as f32),
                    Point::new(horizs.clone().count() as f32 , ch as f32),
                    // TODO verts.clone().count() is not correct
                );
                frame.stroke(&c_line_stroke, stroke_style.clone());
            }

            for cv in verts.clone() {
                let c_line_stroke = canvas::Path::line(
                    Point::new(cv as f32, *horizs.start() as f32),
                    Point::new(cv as f32, verts.clone().count() as f32)
                );

                frame.stroke(&c_line_stroke, stroke_style.clone());
            }
        });

        vec![bg,grid]
    }
}
