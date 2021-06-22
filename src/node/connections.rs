pub type PortID = u32;
pub type NodeID = u32;

#[derive(Debug)]
pub enum PortType {
    Input(u32),
    Output(u32),
}

#[derive(Debug)]
pub struct Addr {
    node: NodeID,
    port: PortID
}
