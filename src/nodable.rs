use super::node_impl::Node;

pub trait Nodable {
    fn new() -> Self;
    fn add_child(&mut self, other: Node);
}
