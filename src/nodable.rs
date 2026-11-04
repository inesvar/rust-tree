use std::{
    cell::{BorrowError, RefCell},
    rc::Rc,
};

pub trait Nodable: Sized {
    type T;

    fn new(value: Rc<RefCell<Self::T>>) -> Self;
    fn add_child(&self, other: Self);
    fn get_parent(&self) -> Result<Option<Self>, BorrowError>;
    fn get_child(&self) -> Result<Option<Self>, BorrowError>;
    fn get_data(&self) -> Result<Rc<RefCell<Self::T>>, BorrowError>;

    fn accumulate_for_all_ancesters<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U;
    fn accumulate_for_all_children<F, U>(&self, init: U, f: F) -> Result<U, BorrowError>
    where
        F: Fn(&Self::T, U) -> U;
}
