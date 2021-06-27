use crate::gui::components::grid::{
    self,
    Drawable,
};

use iced::{
    canvas,
    Rectangle,
    Point,
    Color,
};


#[derive(Debug, Clone)]
pub enum Port {
    Float(f32),
    Unsigned8(u8),
    Integer(i32),
    Array(Vec<Box<Port>>),
}

impl Port {
    const HEIGHT: f32 = 0.5;
}

impl From<&InputPort> for Port {
    fn from(o: &InputPort) -> Port {
        o.port.clone()
    }
}
impl From<&OutputPort> for Port {
    fn from(o: &OutputPort) -> Port {
        o.port.clone()
    }
}


#[derive(Debug, Clone)]
pub struct InputPort {
    disp_name: String,
    port: Port,
}

impl InputPort {
    pub fn new(disp_name: String, port: Port) -> Self {
        InputPort {
            disp_name,
            port,
        }
    }
}

impl Drawable for InputPort {
    fn draw(&self, frame: &mut canvas::Frame) {
        draw_port(&self.port, frame);
    }
    fn get_bounding_box(&self) -> Rectangle {
        get_bounding_box_port()
    }
}

impl From<OutputPort> for InputPort {
    fn from(o: OutputPort) -> InputPort {
        InputPort {
            disp_name: o.disp_name,
            port: o.port,
        }
    }
}


#[derive(Debug, Clone)]
pub struct OutputPort {
    disp_name: String,
    port: Port,
    updated: bool,
}

impl OutputPort {
    pub fn new(disp_name: String, port: Port) -> Self {
        OutputPort {
            disp_name,
            port,
            updated: true,
        }
    }

    pub fn get_updated(&self) -> bool {
        self.updated
    }
}

impl Drawable for OutputPort {
    fn draw(&self, frame: &mut canvas::Frame) {
        draw_port(&self.port, frame);
    }
    fn get_bounding_box(&self) -> Rectangle {
        get_bounding_box_port()
    }
}

impl From<InputPort> for OutputPort {
    fn from(o: InputPort) -> OutputPort {
        Self::new(o.disp_name, o.port)
    }
}



fn get_bounding_box_port() -> Rectangle {
    let mid = Port::HEIGHT/2.;
    Rectangle {
        x: -mid,
        y: -mid,
        width: Port::HEIGHT,
        height: Port::HEIGHT,
    }
}

fn draw_port(port: &Port, frame: &mut canvas::Frame) {
    let color_mul = 0.4;
    let path = canvas::Path::circle(Point::default(), Port::HEIGHT);
    let light_color = match port {
        Port::Array(_) => Color::from_rgb8(0xA0, 0xA0, 0xA0),
        Port::Float(_) => Color::from_rgb8(0xC0, 0x0, 0x0),
        Port::Integer(_) => Color::from_rgb8(0x0, 0xC0, 0x0),
        Port::Unsigned8(_) => Color::from_rgb8(0x0, 0x0, 0xC0),
    };

    let fill_s = Color::from_rgb(
        light_color.r * color_mul,
        light_color.g * color_mul,
        light_color.b * color_mul,
        );
    let stroke_s = canvas::Stroke::default()
        .with_color(light_color)
        .with_width(0.5);

    frame.fill(&path, fill_s);
    frame.stroke(&path, stroke_s);
}
