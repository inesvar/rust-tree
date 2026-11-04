use super::nodable::Nodable;
use super::node_data::NodeData;
use std::cell::{BorrowError, RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct Node<T>(Rc<RefCell<NodeData<T>>>);

impl<T> Nodable for Node<T> {
    type T = T;

    fn new(value: Rc<RefCell<Self::T>>) -> Self {
        Self(Rc::new(RefCell::new(NodeData::new(value))))
    }

    fn add_child(&self, other: Node<T>) {
        other.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow_mut().child = Some(other.0);
    }

    fn get_parent(&self) -> Result<Option<Self>, BorrowError> {
        Ok(self
            .0
            .try_borrow()?
            .parent
            .as_ref()
            .and_then(|opt| opt.upgrade())
            .map(|opt| Self(opt)))
    }

    fn get_child(&self) -> Result<Option<Self>, BorrowError> {
        Ok(self
            .0
            .try_borrow()?
            .child
            .as_ref()
            .map(|opt| Self(opt.clone())))
    }

    fn get_data(&self) -> Result<Rc<RefCell<T>>, BorrowError> {
        Ok(self.0.try_borrow()?.value.clone())
    }

    fn accumulate_for_all_ancesters<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U,
    {
        let mut accumulator = init;
        let data = self.get_data()?;
        let value = data.try_borrow()?;
        accumulator = f(&value, accumulator);

        if let Some(parent) = self.get_parent()? {
            parent.accumulate_for_all_ancesters(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }

    fn accumulate_for_all_children<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U,
    {
        let mut accumulator = init;
        let data = self.get_data()?;
        let value = data.try_borrow()?;
        accumulator = f(&value, accumulator);

        if let Some(child) = self.get_child()? {
            child.accumulate_for_all_children(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }
}
