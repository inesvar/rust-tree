use super::nodable::Nodable;
use super::node_data::NodeData;
use std::{cell::RefCell, rc::Rc};

pub struct Node(Rc<RefCell<NodeData>>);

impl Nodable for Node {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(NodeData::default())))
    }
    fn add_child(&mut self, other: Node) {
        other.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow_mut().child = Some(other.0);
    }
}
