use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct TreeElement {
    parent: Option<Weak<RefCell<TreeElement>>>,
    child: Option<Rc<RefCell<TreeElement>>>,
}

impl Drop for TreeElement {
    fn drop(&mut self) {
        println!("TreeElement dropped");
    }
}

impl TreeElement {
    fn new() -> Rc<RefCell<TreeElement>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            child: None,
        }))
    }

    fn add_child(this: &Rc<RefCell<TreeElement>>, other: Rc<RefCell<TreeElement>>) {
        other.borrow_mut().parent = Some(Rc::downgrade(this));
        this.borrow_mut().child = Some(other);
    }
}

fn main() {
    let root = TreeElement::new();
    TreeElement::add_child(&root, TreeElement::new());

    let weak = Rc::downgrade(&root);
    println!("strong count while we hold root: {}", Rc::strong_count(&root));
    drop(root);
    match weak.upgrade() {
        Some(_) => println!("after drop(root): root still alive (leak)"),
        None => println!("after drop(root): root was freed"),
    }
}
