#![allow(dead_code)]

struct Node<T> {
    e: T,
    next: Option<Box<Node<T>>>,
}

type NodeRef<T> = Box<Node<T>>;

struct List<T> {
    head: Option<NodeRef<T>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, e: T) {
        match self.head.take() {
            None => self.head = Some(Box::new(Node { e, next: None })),
            Some(n) => self.head = Some(Box::new(Node { e, next: Some(n) })),
        }
    } 

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(n) => {
                self.head = n.next;
                Some(n.e)
            }
        }
    }

    fn push_back(&mut self, e: T) {
        let mut tmp = &mut self.head;

        while let Some(n) = tmp {
            tmp = &mut n.next;
        }

        *tmp = Some(Box::new(Node { e, next: None }))
    }

    fn pop_back(&mut self) -> Option<T> {
        let mut tmp: &mut Option<NodeRef<T>>;

        match self.head {
            None => return None,
            Some(_) => tmp = &mut self.head,
        }

        while tmp.as_mut().unwrap().next.is_some() {
            tmp = &mut tmp.as_mut().unwrap().next;
        }

        Some(tmp.take().unwrap().e)
    }
}

fn main() {
    println!("Hello, world!");
}
