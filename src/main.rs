use rust_tree::{Nodable, Node};

fn main() {
    let mut root = Node::new();
    let child = Node::new();
    root.add_child(child);
}
