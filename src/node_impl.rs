use super::node_data::NodeData;
use super::{Nodable, NodeError};
use std::cell::{BorrowError, RefCell};
use std::rc::Rc;

pub struct Node<T>(Rc<RefCell<NodeData<T>>>);

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Nodable for Node<T> {
    type T = T;

    fn new(value: Rc<RefCell<Self::T>>) -> Self {
        Self(Rc::new(RefCell::new(NodeData::new(value))))
    }

    fn is_same_as(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    fn is_ancestor_of(&self, other: &Self) -> Result<bool, BorrowError> {
        if self.is_same_as(other) {
            return Ok(true);
        }

        match other.get_parent()? {
            None => Ok(false),
            Some(parent) => self.is_ancestor_of(&parent),
        }
    }

    fn add_child(&self, other: Self) -> Result<(), NodeError> {
        if other.is_ancestor_of(self)? {
            return Err(NodeError::CycleError);
        }

        if let Some(previous_child) = self.get_child()? {
            previous_child.0.borrow_mut().parent = None;
        }
        self.0.try_borrow_mut()?.child = Some(other.0.clone());
        if let Some(previous_parent) = other.get_parent()? {
            previous_parent.0.borrow_mut().child = None;
        }
        other.0.try_borrow_mut()?.parent = Some(Rc::downgrade(&self.0));
        Ok(())
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

    fn accumulate_for_all_ancestors<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U,
    {
        let mut accumulator = init;
        {
            let data = self.get_data()?;
            let value = data.try_borrow()?;
            accumulator = f(&value, accumulator);
        }
        if let Some(parent) = self.get_parent()? {
            parent.accumulate_for_all_ancestors(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }

    fn accumulate_for_all_children<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U,
    {
        let mut accumulator = init;
        {
            let data = self.get_data()?;
            let value = data.try_borrow()?;
            accumulator = f(&value, accumulator);
        }
        if let Some(child) = self.get_child()? {
            child.accumulate_for_all_children(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }

    fn accumulate_mut_for_all_ancestors<F, U>(&self, init: U, f: F) -> Result<U, NodeError>
    where
        F: Fn(&mut Self::T, U) -> U,
    {
        let mut accumulator = init;
        {
            let data = self.get_data()?;
            let mut value = data.try_borrow_mut()?;
            accumulator = f(&mut value, accumulator);
        }
        if let Some(parent) = self.get_parent()? {
            parent.accumulate_mut_for_all_ancestors(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }

    fn accumulate_mut_for_all_children<F, U>(&self, init: U, f: F) -> Result<U, NodeError>
    where
        F: Fn(&mut Self::T, U) -> U,
    {
        let mut accumulator = init;
        {
            let data = self.get_data()?;
            let mut value = data.try_borrow_mut()?;
            accumulator = f(&mut value, accumulator);
        }

        if let Some(child) = self.get_child()? {
            child.accumulate_mut_for_all_children(accumulator, f)
        } else {
            Ok(accumulator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    fn node() -> Node<i32> {
        Node::new(Rc::new(RefCell::new(0)))
    }

    #[test]
    fn free_node_has_no_parent_and_no_child() -> Result<(), NodeError> {
        let node = node();

        assert!(node.get_parent()?.is_none());
        assert!(node.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_links_two_free_nodes() -> Result<(), NodeError> {
        let parent = node();
        let child = node();

        parent.add_child(child.clone())?;

        assert!(parent.get_child()?.is_some_and(|c| c.is_same_as(&child)));
        assert!(child.get_parent()?.is_some_and(|p| p.is_same_as(&parent)));

        assert!(parent.get_parent()?.is_none());
        assert!(child.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_reparents_and_clears_stale_links() -> Result<(), NodeError> {
        let parent = node();
        let old_child = node();
        parent.add_child(old_child.clone())?;

        let old_parent = node();
        let child = node();
        old_parent.add_child(child.clone())?;

        parent.add_child(child.clone())?;

        assert!(parent.get_child()?.is_some_and(|c| c.is_same_as(&child)));
        assert!(child.get_parent()?.is_some_and(|p| p.is_same_as(&parent)));

        assert!(old_child.get_parent()?.is_none());
        assert!(old_parent.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_is_idempotent_when_readding_existing_child() -> Result<(), NodeError> {
        let parent = node();
        let child = node();
        parent.add_child(child.clone())?;

        parent.add_child(child.clone())?;

        assert!(parent.get_child()?.is_some_and(|c| c.is_same_as(&child)));
        assert!(child.get_parent()?.is_some_and(|p| p.is_same_as(&parent)));

        assert!(parent.get_parent()?.is_none());
        assert!(child.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_rejects_self_cycle() -> Result<(), NodeError> {
        let node = node();

        assert_matches!(node.add_child(node.clone()), Err(NodeError::CycleError));

        assert!(node.get_child()?.is_none());
        assert!(node.get_parent()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_rejects_cycle_when_child_is_ancestor() -> Result<(), NodeError> {
        let parent = node();
        let child = node();
        parent.add_child(child.clone())?;

        assert_matches!(child.add_child(parent.clone()), Err(NodeError::CycleError));

        assert!(parent.get_child()?.is_some_and(|c| c.is_same_as(&child)));
        assert!(child.get_parent()?.is_some_and(|p| p.is_same_as(&parent)));

        assert!(parent.get_parent()?.is_none());
        assert!(child.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_can_move_a_descendant_up_without_cycle() -> Result<(), NodeError> {
        let grand_parent = node();
        let parent = node();
        let child = node();
        parent.add_child(child.clone())?;
        grand_parent.add_child(parent.clone())?;

        grand_parent.add_child(child.clone())?;

        assert!(
            grand_parent
                .get_child()?
                .is_some_and(|c| c.is_same_as(&child))
        );
        assert!(
            child
                .get_parent()?
                .is_some_and(|p| p.is_same_as(&grand_parent))
        );

        assert!(parent.get_parent()?.is_none());
        assert!(parent.get_child()?.is_none());
        Ok(())
    }

    #[test]
    fn add_child_propagates_borrow_errors_without_side_effects() -> Result<(), NodeError> {
        let parent = node();
        let child = node();

        let result = {
            let _guard = parent.0.borrow_mut();
            parent.add_child(child.clone())
        };

        assert_matches!(result, Err(NodeError::BorrowError(_)));
        assert!(parent.get_child()?.is_none());
        assert!(child.get_parent()?.is_none());
        Ok(())
    }
}
