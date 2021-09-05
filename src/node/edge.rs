use super::connections;

#[derive(Debug, Clone)]
pub struct Edges {
    inp: Option<connections::Addr>,
    outp: Vec<connections::Addr>,
}

impl Edges {
    pub fn new() -> Self {
        Self {
            inp: None,
            outp: vec![],
        }
    }

    pub fn set_input(&mut self, inp: connections::Addr) {
        self.inp = Some(inp);
    }

    pub fn get_input(&self) -> Option<connections::Addr> {
        self.inp.clone()
    }

    pub fn add_out(&mut self, outp: connections::Addr) {
        self.outp.push(outp);
    }
    pub fn remove_out(&mut self, outp: connections::Addr) {
        self.outp.retain(|x| *x != outp);
    }
    pub fn get_outputs(&self) -> Vec<connections::Addr> {
        self.outp.clone()
    }
}
