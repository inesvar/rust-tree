use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub(super) struct NodeData<T> {
    pub(super) parent: Option<Weak<RefCell<NodeData<T>>>>,
    pub(super) child: Option<Rc<RefCell<NodeData<T>>>>,
    pub(super) value: Rc<RefCell<T>>,
}

impl<T> NodeData<T> {
    pub(super) fn new(value: Rc<RefCell<T>>) -> Self {
        Self {
            parent: None,
            child: None,
            value,
        }
    }
}
