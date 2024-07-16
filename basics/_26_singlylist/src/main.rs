use banner::print_banner;
use std::fmt::Debug;

type PtrNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    value: T,
    next: PtrNode<T>,
}

#[derive(Debug)]
struct LinkedList<T>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    head: PtrNode<T>,
}

impl<T> LinkedList<T>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    fn create_empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn insert(&mut self, v: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            value: v,
            next: previous_head,
        });

        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old) => {
                self.head = old.next;
                Some(old.value)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(old) => Some(old.value),
            None => None,
        }
    }

    fn print(&self) {
        let mut traveler = &self.head;

        println!("----------printing nodes");
        while true {
            match traveler {
                Some(e) => {
                    print!("'{:?}' => ", e.value);
                    traveler = &e.next;
                }
                Node => {
                    print!("None\n");
                    break;
                }
            }
        }
        println!("-----------end of printing")
    }
}

fn main() {
    print_banner();

    let n = Node {
        value: 3,
        next: Some(Box::new(Node {
            value: 4,
            next: None,
        })),
    };
    let l = LinkedList {
        head: Some(Box::new(n)),
    };

    println!("l.head.value: {:?}", l.head.unwrap().value);

    let mut linked = LinkedList::create_empty();
    linked.insert(3);
    linked.insert(30);
    linked.insert(12);
    linked.insert(4);
    linked.insert(14);

    linked.print();

    println!("linked list :{:?}", linked);

    println!("remove {:?}", linked.remove());
    println!("remove {:?}", linked.remove());
    println!("remove {:?}", linked.remove());
    println!("peek {:?}", linked.peek());

    println!("linked list :{:?}", linked);
    linked.print();

    println!(
        "empty remove: {:?}",
        LinkedList::<i32>::create_empty().remove()
    );
    println!(
        "empty peek : {:?}",
        LinkedList::<i32>::create_empty().peek()
    );

    let mut tuple_list = LinkedList::create_empty();
    println!("tuple list :{:?}", tuple_list);
    tuple_list.insert((32, "Name"));
    tuple_list.insert((12, "User"));
    tuple_list.insert((33, "Second name"));

    println!("tuple list :{:?}", tuple_list);
    println!("tuple peek :{:?}", tuple_list.peek().unwrap());

    tuple_list.print();
}
