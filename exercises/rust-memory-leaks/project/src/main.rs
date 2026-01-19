use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    id: u32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let mut iteration = 0u64;

    loop {
        let node_a = Rc::new(Node {
            id: (iteration * 2) as u32,
            next: RefCell::new(None),
        });

        let node_b = Rc::new(Node {
            id: (iteration * 2 + 1) as u32,
            next: RefCell::new(None),
        });

        *node_a.next.borrow_mut() = Some(Rc::clone(&node_b));
        *node_b.next.borrow_mut() = Some(Rc::clone(&node_a));

        iteration += 1;
    }
}
