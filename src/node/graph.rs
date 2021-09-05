use std::mem::replace;
use std::fmt::Debug;

use crate::gui::components::grid::Drawable;

use iced::{
    canvas,
    Rectangle,
};

pub type NodeIndex = usize;

pub mod err {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub struct GraphError(pub String);

    impl Error for GraphError {}

    impl fmt::Display for GraphError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error in Graph: {}", self.0)
        }
    }
}

pub trait GraphNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph<T>
where T: GraphNode + Debug + Drawable,
{
    nodes: Vec<Option<T>>,
    free_nodes: Vec<NodeIndex>,
    edges: Vec<Option<Edge>>,
    free_edges: Vec<NodeIndex>,

}

impl<T> Graph<T>
where T: GraphNode + Debug + Drawable {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_nodes: Vec::new(),
            edges: Vec::new(),
            free_edges: Vec::new(),
        }
    }

    /// Add a node to the graph. It returns the corresponding id of the node inside the graph.
    /// It takes the Node to be added as an input.
    /// ```
    /// let g = Graph::new();
    /// // Add a new node
    /// let n1 = g.add_node(Node::new("Test".to_owned()));
    /// ```
    pub fn add_node(&mut self, n: T) -> NodeIndex {
        if let Some(id) = self.free_nodes.pop() {
            if let Some(m) = self.nodes.get_mut(id) {
                let _ =  replace(m, Some(n));
            }
            id
        } else {
            self.nodes.push(Some(n));
            self.nodes.len()-1
        }
    }


    /// Get a mutable reference of a node contained in the graph
    /// If None is returned the node doesn't exist.
    /// # Example
    /// ```rust
    /// let g = Graph::new();
    /// let n1 = g.add_node(Node::new("Test".to_owned()));
    ///
    /// // Get the mutable node reference
    /// let mut node = g.get_node_mut(n1).unwrap();
    ///
    /// // Manipulate node inplace
    /// node.name = "No test".to_owned();
    /// ```
    pub fn get_node_mut(&mut self, id: NodeIndex) -> Option<&mut T> {
        self.nodes
            .get_mut(id)
            .and_then(|c| if let Some(n_opt) = c { Some(n_opt) } else { None })
    }

    /// Delete a node from the graph given its id.
    /// It returns an optional Node. If there is no Node with that index `None` is returned.
    /// This function deletes all edges associated with the deleted node.
    /// # Example:
    /// ```rust
    /// // create Graph
    /// let graph = Graph::new();
    /// // Add test node to graph
    /// let node_id = graph.add_node(Node { name: "Test".to_owned() });
    /// let deleted: Node = graph.delete_node(node_id);
    ///
    /// assert_eq!(deleted, Node { name: "Test".to_owned() });
    /// ```
    pub fn delete_node(&mut self, id: NodeIndex) -> Option<T> {
        if id < self.nodes.len() {
            if let Some(current) = self.nodes.get_mut(id) {
                let old_val = replace(current, None);
                self.free_nodes.push(id);
                self.delete_edge_by(Some(id), None);
                // self.delete_edge_by(None, Some(id)); // If implemented as directed graph: delete reversed edges as well 
                old_val
            } else { None }
        } else { None }
    }

    /// Add an edge to the graph given an edge.
    pub fn add_edge(&mut self, e: Edge) -> Result<(), err::GraphError> {
        match (self.nodes.get(e.start), self.nodes.get(e.end)) {
            (Some(_), Some(_)) => {
                if let Some(id) = self.free_edges.pop() {
                    match (self.nodes.get(e.start), self.nodes.get(e.end)) {
                        (Some(_), Some(_)) => {
                            self.edges.insert(id, Some(e));
                            Ok(())
                        },
                        (_, _) => Err(err::GraphError("Not all nodes are existent.".to_owned())),
                    }
                } else {
                    self.edges.push(Some(e));
                    Ok(())
                }
            },
            (Some(_), None) => Err(err::GraphError(format!("Node not existing (End node not existing ({}))", e.end))),
            (None, Some(_)) => Err(err::GraphError(format!("Node not existing (Start node not existing ({}))", e.start))),
            (None, None) => Err(err::GraphError(format!("Nodes not existing (start: {}, end: {})", e.start, e.end))),
        }
    }

    pub fn get_edges(&self) -> Vec<&Edge> {
        self.edges.iter().filter_map(|c| if let Some(c) = c { Some(c) } else { None }).collect()
    }

    pub fn delete_edge_by(&mut self, start: Option<NodeIndex>, end: Option<NodeIndex>) -> Vec<Edge> {

        fn delete_iterator<'a, T, I>(free_edges: &mut Vec<usize>, iter: T) -> Vec<I>
        where
            I: 'a,
            T: Iterator<Item = &'a mut Option<I>>,
        {
            let mut deleted = Vec::new();
            for (i, c) in iter.enumerate() {
                if let Some(replaced) = replace(c, None) {
                    deleted.push(replaced);
                    free_edges.push(i);
                }
            }
            deleted
        }

        match (start, end) {
            (None, None) => delete_iterator(&mut self.free_edges, self.edges.iter_mut()),

            (Some(e), None)|(None, Some(e)) => delete_iterator(&mut self.free_edges, self.edges.iter_mut().filter(|c| if let Some(c) = c {
                c.start == e || c.end == e
            } else {
                false
            })),

            (Some(s), Some(e)) => delete_iterator(&mut self.free_edges, self.edges.iter_mut().filter(|c| if let Some(c) = c {
                (c.start == s && c.end == e) || (c.start == e && c.end == s)
            } else {
                false
            })),
        }
    }


    /// Get the nodes field as non-mutable vector
    pub fn get_nodes(&self) -> Vec<Option<T>> {
        self.nodes
    }
}

impl<T> Drawable for Graph<T>
where T: GraphNode + Debug + Drawable {

    fn draw(&self, frame: &mut canvas::Frame) {
        for c_edge in self.edges {
            if let Some(e) = c_edge {
                e.draw(frame);
            }
        }
        for c_node in self.nodes {
            if let Some(n) = c_node {
                n.draw(frame);
            }
        }
    }

    fn get_bounding_box(&self) -> Rectangle {
        let mut big_bb = None;
        for c_node in self.get_nodes() {
            match (c_node, big_bb) {
                (Some(node), None) => big_bb = Some(node.get_bounding_box()),
                (Some(node), Some(mut s)) => {
                    let cbb = node.get_bounding_box();
                    if cbb.x < s.x {
                        s.x = cbb.x;
                        s.width += s.x - cbb.x;
                    }
                    if cbb.y < s.y {
                        s.y = cbb.y;
                        s.height += s.y - cbb.y;
                    }
                    if (cbb.x + cbb.width) > (s.x + s.width) {
                        s.width = (cbb.x + cbb.width) - s.x;
                    }
                    if (cbb.y + cbb.height) > (s.y + s.height) {
                        s.height = (cbb.y + cbb.height) - s.y;
                    }
                }
                _ => (),
            }
        }
        match big_bb {
            Some(s) => s,
            None => Rectangle::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    start: usize,
    end: usize,
}

impl Edge {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }
}

impl Drawable for Edge {
    fn draw(&self, frame: &mut canvas::Frame) {
        todo!();
    }

    fn get_bounding_box(&self) -> Rectangle {
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use pretty_assertions::{ assert_eq, assert_ne };
    #[derive(Debug, PartialEq)]
    pub struct TestNode(i64);

    impl Drawable for TestNode {
        fn draw(&self, frame: &mut canvas::Frame) {
        }
        fn get_bounding_box(&self) -> iced::Rectangle {
            Rectangle::new(iced::Point::new(0., 0.), iced::Size::new(0., 0.))
        }
    }

    impl GraphNode for TestNode {}

    #[test]
    fn test_new_clean() {
        let g = Graph::<TestNode>::new();

        assert_eq!(g, Graph::<TestNode> {

            nodes: Vec::new(),
            free_nodes: Vec::new(),
            edges: Vec::new(),
            free_edges: Vec::new(),
        });
    }

    #[test]
    fn test_add_node() {
        let mut g = Graph::new();
        let n_id_1 = g.add_node(TestNode(10));
        let n_id_2 = g.add_node(TestNode(100));
        let n_id_3 = g.add_node(TestNode(1000));

        assert_eq!(n_id_1, 0);
        assert_ne!(n_id_1, n_id_2);
        assert_ne!(n_id_1, n_id_3);
        assert_ne!(n_id_2, n_id_3);

        assert_eq!(g, Graph {
            nodes: vec![ 
                Some(TestNode(10)),
                Some(TestNode(100)),
                Some(TestNode(1000)),
            ],
            free_nodes: vec![],
            edges: vec![],
            free_edges: vec![],
        });
    }

    #[test]
    fn test_new_create_use() {
        let mut g = Graph::new();
        let n1 = g.add_node(TestNode(1));
        let n2 = g.add_node(TestNode(2));
        let n3 = g.add_node(TestNode(3));

        let _e1 = g.add_edge(Edge { start: n1, end: n2 });
        let _e2 = g.add_edge(Edge { start: n3, end: n1 });

        assert_eq!(g, Graph {
            nodes: vec![ 
                Some(TestNode(1)),
                Some(TestNode(2)),
                Some(TestNode(3))
            ],
            free_nodes: vec![],
            edges: vec![
                Some(Edge {start: 0, end: 1}),
                Some(Edge {start: 2, end: 0}),
            ],
            free_edges: vec![],
        });
    }


    #[test]
    fn test_delete_node() {
        let mut g = Graph::new();
        let n1 = g.add_node(TestNode(-3));
        let n2 = g.add_node(TestNode(-2));
        let n3 = g.add_node(TestNode(-1));

        let _e1 = g.add_edge(Edge { start: n1, end: n2 });
        let _e2 = g.add_edge(Edge { start: n3, end: n1 });

        let del = g.delete_node(n2).unwrap();

        assert_eq!(del, TestNode(-2));

        assert_eq!(g, Graph {
            nodes: vec![ 
                Some(TestNode(-3)),
                None,
                Some(TestNode(-1))
            ],
            free_nodes: vec![1],
            edges: vec![
                None,
                Some(Edge {start: 2, end: 0}),
            ],
            free_edges: vec![0],
        });
    }

    #[test]
    fn test_delete_after_add() {
        let mut g = Graph::new();
        let n1 = g.add_node(TestNode(1));
        let _n2 = g.add_node(TestNode(2));

        let deleted = g.delete_node(n1).unwrap();
        assert_eq!(deleted, TestNode(1));

        let _n3 = g.add_node(TestNode(3));

        assert_eq!(g, Graph {
            nodes: vec![ 
                Some(TestNode(3)),
                Some(TestNode(2)),
            ],
            free_nodes: vec![],
            edges: vec![],
            free_edges: vec![],
        });
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        let n_id_1 = g.add_node(TestNode(1));
        let n_id_2 = g.add_node(TestNode(2));

        assert_ne!(Ok(()), g.add_edge(Edge { start: n_id_1, end: 10 }));
        assert_eq!(
            g,
            Graph {
                nodes: vec![ 
                    Some(TestNode(1)),
                    Some(TestNode(2)),
                ],
                free_nodes: vec![],
                edges: vec![],
                free_edges: vec![],
            }
        );

        assert_eq!(Ok(()), g.add_edge(Edge { start: n_id_1, end: n_id_2 }));
        assert_eq!(
            g,
            Graph {
                nodes: vec![ 
                    Some(TestNode(1)),
                    Some(TestNode(2)),
                ],
                free_nodes: vec![],
                edges: vec![
                    Some(Edge { start: n_id_1, end: n_id_2 }),
                ],
                free_edges: vec![],
            }
        );
    }

    #[test]
    fn test_delete_edge() {
        let mut g = Graph::new();
        let n1 = g.add_node(TestNode(1));
        let n2 = g.add_node(TestNode(2));
        let n3 = g.add_node(TestNode(3));

        assert_eq!(Ok(()), g.add_edge(Edge::new(n1, n2)));
        assert_eq!(Ok(()), g.add_edge(Edge::new(n1, n3)));
        assert_eq!(Ok(()), g.add_edge(Edge::new(n2, n3)));

        assert_ne!(Ok(()), g.add_edge(Edge::new(n1, 10)));

        assert_eq!(g.delete_edge_by(Some(n1), None), vec![
            Edge::new(n1, n2),
            Edge::new(n1, n3),
        ]);
        assert_eq!(g,  Graph {
            nodes: vec![ 
                Some(TestNode(1)),
                Some(TestNode(2)),
                Some(TestNode(3))
            ],
            free_nodes: vec![],
            edges: vec![
                None,
                None,
                Some(Edge::new(n2, n3)),
            ],
            free_edges: vec![
                0,
                1,
            ],
        });

        // Readd n1-Edges
        assert_eq!(Ok(()), g.add_edge(Edge::new(n1, n2)));
        assert_eq!(Ok(()), g.add_edge(Edge::new(n1, n3)));

        assert_ne!(g.delete_edge_by(None, Some(n1)), vec![
            Edge::new(n1, n2),
            Edge::new(n1, n3),
        ]);

        assert_eq!(g.delete_edge_by(Some(n1), Some(10)), vec![]);
    }
}
