use std::cell::RefCell;
use std::rc::Rc;
use banner::print_banner;

struct Node<T> {
    element: T,
    next: PtrPointer<T>,
    prev: PtrPointer<T>,
}

struct LinkedList<T> {
    head: PtrPointer<T>,
    tail: PtrPointer<T>,
}

type PtrPointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T>
where
    T: std::fmt::Display,
{
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}

impl<T> LinkedList<T>
where
    T: std::fmt::Display,
{
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, v: T) {
        let node = Node::new(v);
        match self.head.take() {
            Some(e) => {
                e.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(e.clone());
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());

                self.head = Some(node);
            }
        }
    }

    fn push_back(&mut self, v: T) {
        let node = Node::new(v);
        match self.tail.take() {
            Some(e) => {
                e.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(e.clone());
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    fn print(&self) {
        println!("pringlink linked list");

        /*while true {
            let mut head = self.head.take();
            match head {
                None => {
                    break
                }
                Some(a) => {
                    // print!(":{:?}: =>", a.borrow());
                    head  = a.borrow().next;
                }
            }
        }*/
    }
}

fn main() {
    print_banner();

    let mut l: LinkedList<i32> = LinkedList::new();
    l.push_front(3);
    l.push_front(1);
    l.push_back(55);
    l.push_back(43);
}
