use banner::print_banner;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("dropping node {:?}", self);
    }
}

fn main() {
    print_banner();

    let a = Rc::new(RefCell::new(Node { next: None }));
    println!(
        "a. count {}, week {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&a)),
    }));
    println!(
        "a. count {}, week {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b. count {}, week {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );

    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::downgrade(&b)),
    }));
    println!(
        "a. count {}, week {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b. count {}, week {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!(
        "c. count {}, week {}",
        Rc::strong_count(&c),
        Rc::weak_count(&c)
    );

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));
    println!("a attach to c");
    println!(
        "a. count {}, week {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b. count {}, week {}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!(
        "c. count {}, week {}",
        Rc::strong_count(&c),
        Rc::weak_count(&c)
    );
    // where is the drop ??

    // will fail if rc::clone . but works with weak pointer
    println!("a: {:?}", a);

    let leaf = Rc::new(TreeNode {
        element: 73,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        element: 43,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}

#[derive(Debug)]
struct TreeNode {
    element: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}
