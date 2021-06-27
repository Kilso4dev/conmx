use std::fmt;
use log::info;

use super::port::{
    Port,
    InputPort,
    OutputPort,
};

use crate::gui::components::grid::{
    self,
    helpers,
};

use iced::{
    canvas,
    Point,
    Color,
    Vector,
};

use super::err;

pub type DriverFunction = fn(&Vec<InputPort>) -> Vec<OutputPort>;

#[derive(Clone)]
pub struct Node {
    position: Point,

    inputs: Vec<InputPort>,
    outputs: Vec<OutputPort>,

    drivers: Vec<DriverFunction>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Node {{ inputs: {:?}, outputs: {:?}, driver count: {} }}", self.inputs, self.outputs, self.drivers.len())
    }
}

impl Node {
    const NODE_RADIUS: f32 = 1.;
    const HEADER_HEIGHT: f32 = 2.;
    const PORT_HEIGHT: f32 = 1.5;
    const FOOTER_HEIGHT: f32 = 2.;
    const NODE_WIDTH: f32 = 10.;

    pub fn new() -> NodeBuilder {
        NodeBuilder::new()
    }

    pub fn udpate(&mut self) {
        for c_driver in self.drivers.iter() {
            let _changed_ins = c_driver(&self.inputs);
        }

        // TODO implement
    }

    pub fn is_updated(&self, id: usize) -> bool {
        match self.outputs.get(id) {
            Some(out_port) => out_port.get_updated(),
            None => false,
        }
    }

    pub fn set_pos(&mut self, new_pos: Point) {
        self.position = new_pos;
    }

    pub fn translate(&mut self, translation: Vector) {
        self.position = self.position + translation;
    }

    fn calculate_height(&self) -> f32 {
        Self::HEADER_HEIGHT +
            ((self.inputs.len().max(self.outputs.len()) as f32) * Self::PORT_HEIGHT) +
            Self::FOOTER_HEIGHT
    }

}
impl grid::Drawable for Node {

    fn draw(&self, frame: &mut canvas::Frame) {
        let path = helpers::rounded_rect_path(self.get_bounding_box(), Self::NODE_RADIUS);

        let fill = iced::Color::from_rgb8(0x0F, 0x0F, 0x0F);
        let stroke_s =  canvas::Stroke::default()
            .with_color(Color::from_rgb8(0x00, 0x80, 0x00))
            .with_width(2.);

        frame.stroke(&path, stroke_s);
        frame.fill(&path, fill);

        let mut c_transl = Vector::new(0., Self::HEADER_HEIGHT + (Self::PORT_HEIGHT/2.));
        for cin_port in self.inputs.iter() {
            frame.translate(c_transl);
            cin_port.draw(frame);
            frame.translate(Vector::default()-c_transl);
            c_transl = c_transl + Vector::new(0., Self::PORT_HEIGHT);
        }


        let mut c_transl = Vector::new(Self::NODE_WIDTH, Self::HEADER_HEIGHT + (Self::PORT_HEIGHT/2.));
        for cout_port in self.outputs.iter() {
            frame.translate(c_transl);
            cout_port.draw(frame);
            frame.translate(Vector::default()-c_transl);
            c_transl = c_transl + Vector::new(0., Self::PORT_HEIGHT);
        }
    }

    fn get_bounding_box(&self) -> iced::Rectangle {
        iced::Rectangle::new(self.position.clone(), iced::Size {
            width: Self::NODE_WIDTH,
            height: self.calculate_height(),
        })
    }
}


pub struct NodeBuilder {
    starting_pos: Point,
    inputs: Vec<InputPort>,
    outputs: Vec<InputPort>,
    drivers: Vec<DriverFunction>,
}

impl NodeBuilder {
    fn new() -> Self {
        Self {
            starting_pos: Point::default(),
            inputs: Vec::with_capacity(20),
            outputs: Vec::with_capacity(10),
            drivers: Vec::with_capacity(10),
        }
    }

    pub fn with_starting_pos(mut self, pos: Point) -> Self {
        self.starting_pos = pos;
        self
    }

    pub fn with_in(mut self, name: String, port: Port) -> Self {
        self.inputs.push(InputPort::new(name, port));
        self
    }

    pub fn with_out(mut self, name: String, port: Port) -> Self {
        self.outputs.push(InputPort::new(name, port));
        self
    }

    pub fn with_driver(mut self, new_driver: DriverFunction) -> Self {
        self.drivers.push(new_driver);
        self
    }

    pub fn build(self) -> Result<Node, err::NodeCreationErr> {

        // TODO Check if drivers overlap and warn appropiatly

        Ok(Node {
            position: self.starting_pos,
            inputs: self.inputs,
            outputs: self.outputs
                .into_iter()
                .map(OutputPort::from)
                .collect(),
            drivers: self.drivers,
        })
    }
}
