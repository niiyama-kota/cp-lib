pub mod list {
    use std::{mem, io::Empty};

    #[derive(Debug)]
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    #[derive(Debug)]
    enum Link<T> {
        Empty,
        More(Box<Node<T>>),
    }
    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }

        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem: elem,
                next: mem::replace(&mut self.head, Link::Empty),
            });
        }
    }
}

#[test]
fn test_list_add_fn() {
    use list::List;
    let mut list: List<i32> = List::new();
    list.push(0);
    list.push(1);
    println!("{:?}", list);
}
