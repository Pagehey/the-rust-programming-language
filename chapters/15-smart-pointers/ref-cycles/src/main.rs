use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new(
        Node { value: 5, parent: RefCell::new(Weak::new()),  children: RefCell::new(vec![]) }
    );

    println!("{:?}", leaf);

    let branch = Rc::new(
        Node { value: 5, parent: RefCell::new(Weak::new()),  children: RefCell::new(vec![Rc::clone(&leaf)]) }
    );

    println!("{:?}", leaf);
    println!("{:?}", branch);

   *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}
