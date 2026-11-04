use rust_tree::{Nodable, Node};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let world_transform = Rc::new(RefCell::new(1));
    let planet_transform = Rc::new(RefCell::new(1));
    let player_transform = Rc::new(RefCell::new(1));

    let world_node = Node::<i32>::new(world_transform.clone());
    let planet_node = Node::<i32>::new(planet_transform.clone());
    let player_node = Node::<i32>::new(player_transform.clone());

    planet_node.add_child(player_node.clone());
    world_node.add_child(planet_node.clone());

    let result = player_node.accumulate_for_all_ancesters(1, |init, acc| init * acc);
    println!("{:?}", result);

    *world_transform.borrow_mut() = 2;
    *player_transform.borrow_mut() = 4;

    let result = player_node.accumulate_for_all_ancesters(1, |init, acc| init * acc);
    println!("{:?}", result);

    let result = world_node.accumulate_for_all_children(1, |init, acc| init * acc);
    println!("{:?}", result);

    let result = world_node.accumulate_mut_for_all_children(1, |init, acc| {
        *init *= 2;
        *init * acc}
    );
    println!("{:?}", result);

    let mut truc = world_transform.borrow_mut();
    let result = player_node.accumulate_for_all_ancesters(1, |init, acc| init * acc);
    println!("{:?}", result);
    *truc = 1;
}
