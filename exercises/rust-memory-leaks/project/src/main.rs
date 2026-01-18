use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    id: u32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    println!("Starting CONTINUOUS memory leak demonstration...");
    println!("Creating node pairs with circular references indefinitely.");
    println!("Watch memory usage grow - these nodes will never be deallocated!");
    println!("Press Ctrl+C to stop the program.\n");

    let mut iteration = 0u64;

    loop {
        // Create two nodes in each iteration
        let node_a = Rc::new(Node {
            id: (iteration * 2) as u32,
            next: RefCell::new(None),
        });

        let node_b = Rc::new(Node {
            id: (iteration * 2 + 1) as u32,
            next: RefCell::new(None),
        });

        // Create the cycle: A -> B and B -> A
        // This is the bug! The reference counts will never drop to zero.
        *node_a.next.borrow_mut() = Some(Rc::clone(&node_b));
        *node_b.next.borrow_mut() = Some(Rc::clone(&node_a));

        // Both node_a and node_b go out of scope here,
        // but the Rc reference counts are still 1 due to the cycle!
        // Memory is never deallocated - it just grows and grows...

        iteration += 1;

        // Print progress every 10,000 iterations
        if iteration % 10_000 == 0 {
            println!("Created {} node pairs ({} nodes total)...", iteration, iteration * 2);
        }
    }
}
