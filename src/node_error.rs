use std::cell::{BorrowError, BorrowMutError};

#[derive(Debug)]
pub enum NodeError {
    BorrowError(BorrowError),
    BorrowMutError(BorrowMutError),
    CycleError,
}

impl From<BorrowError> for NodeError {
    fn from(value: BorrowError) -> Self {
        NodeError::BorrowError(value)
    }
}

impl From<BorrowMutError> for NodeError {
    fn from(value: BorrowMutError) -> Self {
        NodeError::BorrowMutError(value)
    }
}
