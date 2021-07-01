use log::warn;

use iced::{
    mouse,
    Length,
    canvas,
    Element,
    Point,
    Color,
    Size,
    Vector,
};

use log::info;

use super::{
    helpers,
    drawable::Drawable,
};


pub enum Msg {
    PosChange(Point),
    ScaleChange(f32, Option<iced::Vector>),
}

#[derive(Debug, Default)]
pub struct Grid {
    mouse_last_pos: Option<iced::Point>,
    mouse_drag_node: bool,
    mouse_drag_screen: bool,

    scale_sensivity: f32,
    translation: Vector,
    scaling: f32,
    grid_dist: f32,

    overlay_cache: canvas::Cache,
    connection_cache: canvas::Cache,
    node_cache: canvas::Cache,
    background_cache: canvas::Cache,
}

impl Grid {
    const MIN_SCALING: f32 = 1.;
    const MAX_SCALING: f32 = 100.;

    pub fn new() -> Self{
        Self {
            scale_sensivity: 20.,

            translation: Vector::new(0., 0.),
            scaling: 10.,
            grid_dist: 1.,

            overlay_cache: canvas::Cache::new(),
            background_cache: canvas::Cache::new(),

            ..Self::default()
        }
    }

    pub fn view<'a>(&'a mut self) -> Element<'a, Msg> {
        canvas::Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn calc_visible_region(&self, size: Size, grid_dist: f32) -> helpers::Region {
        let w = size.width / (self.scaling);
        let h = size.height / (self.scaling);
        //info!("Scaled w: {}, h: {}", w, h);
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

    fn project_vec(&self, v: Vector) -> Vector {
        Vector::new(v.x / self.scaling, v.y / self.scaling)
    }

    fn translate_viewport(&self, frame: &mut canvas::Frame, bounds: &iced::Rectangle) {
        frame.translate(Vector::new(bounds.width / 2.0, bounds.height / 2.0));
        frame.scale(self.scaling);
        frame.translate(self.translation);
        frame.scale(self.grid_dist);
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::PosChange(_p) => {
            }
            Msg::ScaleChange(_new_scale, _transl) => {
            }
        }
    }

    fn clear_caches(&mut self) {
        self.background_cache.clear();
        self.node_cache.clear();
        self.connection_cache.clear();
        self.overlay_cache.clear();
    }

    fn draw_bg(&self, frame: &mut canvas::Frame, color: Color) {
        let bg = canvas::Path::rectangle(iced::Point::ORIGIN, frame.size());

        frame.fill(&bg, color);
    }

    fn draw_grid(&self,
        frame: &mut canvas::Frame,
        color: Color,
        bounds: &iced::Rectangle) {

        self.translate_viewport(frame, bounds);

        let line_width = 1.;

        let region = self.calc_visible_region(frame.size(), self.grid_dist);
        let verts = region.vert_lines();
        let horizs = region.horiz_lines();

        let circle_path = canvas::Path::circle(Point::new(0., 0.), 0.8);

        let stroke_style = canvas::Stroke::default()
            .with_color(color)
            .with_width(line_width);


        frame.stroke(&circle_path, stroke_style.clone());

        // draw horizontal lines
        for ch in horizs.clone() {
            let c_line_stroke = canvas::Path::line(
                Point::new(*verts.start() as f32, ch as f32),
                Point::new(*verts.end() as f32, ch as f32),
            );
            frame.stroke(&c_line_stroke, stroke_style.clone());
        }

        // draw vertical lines
        for cv in verts.clone() {
            let c_line_stroke = canvas::Path::line(
                Point::new(cv as f32, *horizs.start() as f32),
                Point::new(cv as f32, *horizs.end() as f32)
            );
            frame.stroke(&c_line_stroke, stroke_style.clone());
        }
    }
}

impl<'a> canvas::Program<Msg> for Grid {
    fn update(
        &mut self,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: canvas::Cursor,
    ) -> ( canvas::event::Status, Option<Msg> ) {
        let cursor_pos = 
            if let Some(position) = cursor.position_in(&bounds) {
                position
            } else {
                return (canvas::event::Status::Ignored, None);
            };
        //info!("Event: {:?}, bounds: {:?}, cursor: {:?}", event, bounds, cursor);

        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse_but) => {
                    match mouse_but {
                        mouse::Button::Middle => {
                            self.mouse_drag_screen = true;
                            (canvas::event::Status::Captured, None)
                        }
                        mouse::Button::Left => {
                            self.mouse_drag_node = true;
                            (canvas::event::Status::Captured, None)
                        }
                        _ => (canvas::event::Status::Ignored, None)
                    }       
                },
                mouse::Event::ButtonReleased(mouse_but) => {
                    match mouse_but {
                        mouse::Button::Middle => {
                            self.mouse_drag_screen = false;
                            self.mouse_last_pos = None;
                            (canvas::event::Status::Captured, None)
                        }
                        mouse::Button::Left => {
                            self.mouse_drag_node = false;
                            (canvas::event::Status::Captured, None)
                        }
                        _ => (canvas::event::Status::Ignored, None)
                    }
                },
                mouse::Event::CursorMoved{ .. } => {
                    if let Some(last_pos) = self.mouse_last_pos {
                        if self.mouse_drag_screen {
                            let pos_delta = cursor_pos - last_pos;
                            self.translation = self.translation + self.project_vec(pos_delta);
                            self.clear_caches();
                        }
                        if self.mouse_drag_node {

                        }
                    }
                    self.mouse_last_pos = Some(cursor_pos);
                    (canvas::event::Status::Ignored, None)
                },
                mouse::Event::WheelScrolled{delta} => {
                    match delta {
                        mouse::ScrollDelta::Lines{ y, .. }|
                        mouse::ScrollDelta::Pixels{ y, .. } => {
                            let new_scale = (self.scaling * (1.0 + y / self.scale_sensivity))
                                .min(Self::MAX_SCALING)
                                .max(Self::MIN_SCALING);
                            let old_scaling = self.scaling;
                            info!("new scaling: {}", new_scale);

                            if let Some(cursor_to_center) =
                                cursor.position_from(bounds.center()) {
                                let fac = new_scale - old_scaling;

                                let new_transl = self.translation - iced::Vector::new(
                                    cursor_to_center.x * fac / (old_scaling * old_scaling),
                                    cursor_to_center.y * fac / (old_scaling * old_scaling),
                                );

                                self.translation = new_transl;
                            }
                            self.scaling = new_scale;

                            self.clear_caches();
                            (canvas::event::Status::Captured, None)
                        }
                    }
                },
                _ => (canvas::event::Status::Ignored, None)
            }
            _ => (canvas::event::Status::Ignored, None)
        }
    }

    fn draw(
        &self,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {


        let bg = self.background_cache.draw(bounds.size(), |frame| {
            let color_light_mul = 1.3;

            let bg_color = Color::from_rgb8(0x22, 0x22, 0x22);
            let bg_color_light = Color::from_rgb(
                bg_color.r * color_light_mul, 
                bg_color.g * color_light_mul,
                bg_color.b * color_light_mul);

            self.draw_bg(frame, bg_color);
            self.draw_grid(frame, bg_color_light, &bounds);
        });

        let nodes = self.node_cache.draw(bounds.size(), |frame| {
            use crate::node::{
                Node,
                Port,
            };
            self.translate_viewport(frame, &bounds);

            let node = Node::new()
                .with_in("Input".to_owned(), Port::Float(0.))
                .with_in("IN 2".to_owned(), Port::Unsigned8(0))
                .with_in("IN 3".to_owned(), Port::Integer(0))
                .with_in("IN 4".to_owned(), Port::Array(vec![]))
                .with_out("Output".to_owned(), Port::Float(0.))
                .with_driver()
                .build();

            match node {
                Ok(node) => node.draw(frame),
                Err(e) => warn!("Example node could not be constructed: {}", e),
            }
        });

        let connections = self.connection_cache.draw(bounds.size(), |frame| {
        });

        let overlay = {
            let mut frame = canvas::Frame::new(bounds.size());

            let text = canvas::Text {
                color: Color::WHITE,
                size: 20.,
                position: Point::new(frame.width(), frame.height()),
                horizontal_alignment: iced::HorizontalAlignment::Right,
                vertical_alignment: iced::VerticalAlignment::Bottom,
                ..canvas::Text::default()
            };

            frame.fill_text(canvas::Text {
                content: format!("drag_node: {:6} drag_screen: {:6} translation: {:2.3?}",
                    self.mouse_drag_node,
                    self.mouse_drag_screen,
                    self.translation,
                    ),
                ..text
            });

            frame.into_geometry()
        };



        vec![bg, connections, nodes, overlay]
    }
}

