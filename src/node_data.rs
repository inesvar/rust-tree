use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub(super) struct NodeData {
    pub(super) parent: Option<Weak<RefCell<NodeData>>>,
    pub(super) child: Option<Rc<RefCell<NodeData>>>,
}
