use std::collections::HashMap;

use super::node::Node;
use super::connections::NodeID;

#[derive(Debug)]
pub struct NodeTree {
    calc_hooks: Option<()>,
    nodes: HashMap<NodeID, Node>,
}

impl NodeTree {
}
