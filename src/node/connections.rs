use std::cmp::{
    PartialEq,
    Eq,
};

pub type PortID = usize;
pub type NodeID = usize;

#[derive(Debug, Clone)]
pub enum PortType {
    Input(u32),
    Output(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Addr {
    pub node: NodeID,
    pub port: PortID
}

impl PartialEq for Addr {
    fn eq(&self, o: &Addr) -> bool {
        self.node == o.node && self.port == o.port
    }
}

impl Eq for Addr {}
