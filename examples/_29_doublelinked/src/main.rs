use banner::print_banner;
use std::cell::RefCell;
use std::rc::Rc;

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
            element,
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

    fn remove_front(&mut self) {
        if self.head.is_some() {
            self.head
                .take()
                .map(|old| match old.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail.take();
                        None
                    }
                });
        }
    }

    fn remove_back(&mut self) {
        if self.tail.is_some() {
            self.tail
                .take()
                .map(|old_tail| match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head.take();
                        None
                    }
                });
        }
    }

    fn print(&self) {
        println!("pringlink linked list");
        if self.head.is_none() {
            println!("| ");
        } else {
            let mut traveler = self.head.clone();
            while !traveler.is_none() {
                print!("{} => ", traveler.as_ref().unwrap().borrow().element);
                traveler = traveler.unwrap().borrow().next.clone();
            }
            println!(" | ");
        }

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

    l.print();

    l.remove_front();
    l.print();

    l.remove_back();
    l.print();
    l.remove_front();
    l.remove_front();
    l.remove_back();
    l.remove_front();
    l.remove_back();
    l.print();

    //println!("list {:?}",l);
}
