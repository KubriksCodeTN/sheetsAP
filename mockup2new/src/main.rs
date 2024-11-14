#![allow(dead_code)]

/*
Write a struct `BinIter` that implements `Iterator` over `bool`s.

- `BinIter` must have a function `new` that takes as input `n` the number and `l` the length.

- The iterator must yield bits according to the binary form of `n`, after returning the `l`-th bit the iterator stops.

- The bits yielded must be in "little-endian" order, so the most significant bit must be yielded last.
 */

use std::cell::RefCell;

struct BinIter {
    n: u128,
    l: usize,
}

impl BinIter {
    fn new(n: u128, l: usize) -> Self {
        Self { n, l }
    }
}

impl Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.l == 0 {
            return None
        }
        let ret = Some(self.n & 1 == 1);
        self.l -= 1;
        self.n >>= 1;
        ret
    }
}

#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display};

/* */
/*
Implement a doubly linked list

Create the necessary structs to represent it

- Node<T> with an element of type T and two fields, prev and next, both of type Option<Rc<RefCell<Node<T>>>>.

- List<T> with two fields, head and tail, both of type Option<Rc<RefCell<Node<T>>>>, and a size field of type usize.

Implement the following traits for Node<T>:

- PartialEq that compares the elements of two nodes.

- Display that prints the element of a node.

Implement the following traits for List<T>:

- PartialEq that checks if two lists are equal, by comparing the elements of the nodes, one by one.

- Debug that prints the elements of the list.

Implement the following methods for List<T>:

- new() that creates a new empty list.

- print_list(&self) that prints the elements of the list.

- push(&mut self, element: T) that adds an element to the front of the list.

- pop(&mut self) -> Option<T> that removes an element from the front of the list.

- push_back(&mut self, element: T) that adds an element to the back of the list.

- pop_back(&mut self) -> Option<T> that removes an element from the back of the list.

- print_list(&self) that prints the elements of the list.
 */
use std::rc::Rc;
type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    element: T, 
    prev: NodeRef<T>,
    next: NodeRef<T>,
}

impl<T> Node<T> {
    fn new(e: T) -> Self {
        Self { element: e, prev: None, next: None }
    }
}

struct List<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
    size: usize,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl<T: Display> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        let mut tmp = self.head.clone();
        let mut tmp1 = other.head.clone();

        while let Some(n) = tmp {
            let ret = tmp1.unwrap();
            if n.borrow().element !=  ret.borrow().element {
                return false;
            }
            tmp = n.borrow().next.clone();
            tmp1 = ret.borrow().next.clone();
        }

        true
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut currently_head = self.head.clone();
        while let Some(node) = currently_head {
            write!(f, "{}", node.borrow().element)?;
            currently_head = node.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = self.head.clone();
        while let Some(n) = tmp {
            let _ = write!(f, "{:?}\n", n.borrow().element);
            tmp = n.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    fn push(&mut self, e: T) {
        match self.head.take() {
            None => {
                let new_n = Rc::new(RefCell::new(Node::new(e)));
                self.tail = Some(new_n.clone());
                self.head = Some(new_n);
            }
            Some(n) => {
                let new_n = Rc::new(RefCell::new(Node::new(e)));
                new_n.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(new_n.clone());
                self.head = Some(new_n);
            }
        }
        self.size += 1;
    }    

    fn push_back(&mut self, e: T) {
        let new_n = Rc::new(RefCell::new(Node::new(e)));
        match self.tail.take() {
            None => {
                self.head = Some(new_n.clone());
                self.tail = Some(new_n);
            }
            Some(n) => {
                n.borrow_mut().next = Some(new_n.clone());
                new_n.borrow_mut().prev = Some(n.clone());
                self.tail = Some(new_n);
            }
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(n) => {
                match n.borrow_mut().next.take() {
                    Some(n1) => {
                        n1.borrow_mut().prev = None;
                        self.head = Some(n1);
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                self.size -= 1;
                Some(Rc::try_unwrap(n).ok().unwrap().into_inner().element)
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(n) => {
                match n.borrow_mut().prev.take() {
                    None => {
                        self.head = None;
                        self.tail = None;
                   }
                   Some(n1) => {
                        n1.borrow_mut().next = None;
                        self.tail = Some(n1);
                   }
                }
                self.size -= 1;
                Some(Rc::try_unwrap(n).ok().unwrap().into_inner().element)
            }
        }
    }
} 

impl<T: Display> List<T> {
    fn print_list(&self) {
        let mut tmp = self.head.clone();
        while let Some(ref n) = tmp {
            println!("{}", n.borrow());
            tmp = tmp.unwrap().borrow().next.clone();
        }
    }
}
/* */
/*
Write a trait `Task` that define a method `execute(&self)->usize`.

implement the `Task` trait for the following structs:

- `SumTask` is a struct with a method `new(n1: usize, n2: usize)` were executing task returns

the sum of n1 and n2

- LenTask is a struct with a method `new(s: String)` were executing task returns

the len of s

Write two structs: `Tasker` and `Executer`, that interact following this protocol:

- At any given time any number of tasker and executer can be linked together.

- `Tasker` can ask for a task to be scheduled using the method `schedule_task(&mut self, task: ...)` that take as input a

box with inside an object that implements Task

- `Executer` can execute a task using the method execute_task(&mut self)->Option<usize>. this method can fail if no task is scheduled

- Tasks are executed inf a FIFO queue

- `Tasker` has a method `new` that return am instance with an empty queue, linked to no one.

- `Tasker` has a method `get_tasker(&self)->Tasker` that return a new `Tasker` linked with self.

- `Tasker` has a method `get_executer(&self)->Executer` that return a new `Executer` linked with self.
 */

trait Task {
    fn execute(&self) -> usize;
}

struct SumTask {
    n1: usize,
    n2: usize,
}

impl SumTask {
    fn new(n1: usize, n2: usize) -> Self {
        Self { n1, n2 }
    }
}

impl Task for SumTask {
    fn execute(&self) -> usize {
        self.n1 + self.n2
    }
}

struct LenTask {
    s: String,
}

impl LenTask {
    fn new(s: String) -> Self {
        Self { s }
    }
}

impl Task for LenTask {
    fn execute(&self) -> usize {
        self.s.len()
    }
}

struct Tasker {
    l: Rc<RefCell<VecDeque<Box<dyn Task>>>>,
}

struct Executer {
    l: Rc<RefCell<VecDeque<Box<dyn Task>>>>,
}

impl Tasker {
    fn new() -> Self {
        Self { l: Rc::new(RefCell::new(VecDeque::new())) }
    }

    fn get_tasker(&self) -> Tasker {
        Tasker { l: self.l.clone() }
    }

    fn get_executer(&self) -> Executer {
        Executer { l: self.l.clone() }
    }

    fn schedule_task(&mut self, task: Box<dyn Task>) {
        self.l.borrow_mut().push_back(task);
    }
}

impl Executer {
    fn execute_task(&mut self) -> Option<usize> {
        if self.l.borrow().len() == 0 {
            return None;
        }
        Some(self.l.borrow_mut().pop_front().unwrap().execute())
    }
}

/* */
/*
define an i32 constant named "CONSTANT" inside a module named "odd_module" and assign to it the value 123

define an i32 constant named "CONSTANT" inside a module named "even_module" and assign to it the value 246

define a public function "get_constant" inside the module "getter_function" that take as input an u32 named "value", and return

the constant inside "odd_module" if "value" is odd. otherwise it returns the constant inside "even_module"

just to avoid confusion remember that in Italian: odd = dispari, even = pari
 */

 mod odd_module {
    pub const CONSTANT: i32 = 123;
 }

 mod even_module {
    pub const CONSTANT: i32 = 246;
 }

 mod getter_function {
    use crate::{even_module, odd_module};

    fn get_constant(value: i32) -> i32 {
        if value % 2 == 1 {
            return odd_module::CONSTANT;
        }
        return even_module::CONSTANT;
    }
 }

 /* */
/*
define a trait CloneAndDouble with a function `clone_and_double(&self)->Self`

the function clone_and_double clone the item and double it.

Implement the trait for all items that implement the traits Clone and Add (use a simple addition to double)
 */

trait CloneAndDouble {
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + std::ops::Add<Output = T>> CloneAndDouble for T {
    fn clone_and_double(&self) -> Self {
        self.clone() + self.clone()
    }
}

/* */
/*
The trait `Unknown` defines a method `serialize` that returns the implementer's `String` representation.

- [1] implement it for `i32`

- [1] implement it for `String`

- [3] implement it for `Vec<T>`, where T implements Debug

- [2] write a function `get_vec` that returns an empty vec of `Unknown` data

- [3] write a function `print_vec` that takes as input a reference of a vec of `Unknown` data and prints its content

```
*/
trait Unknown {

    fn serialize(&self) -> String;

}

impl Unknown for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T: Debug> Unknown for Vec<T> {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    Vec::new()
}

fn print_vec(v: &Vec<Box<dyn Unknown>>) {
    println!("{:?}", v.iter().map(|s| s.serialize()).collect::<Vec<String>>())
}

/* */
/*
Write the necessary structs to represent an oriented graph generic over `T`, where `T`

implements `Hash`, `PartialEq` and `Eq`.

- `Node`, with a value of type `T` and a vector of adjacent nodes

- `Graph`, with a vector of nodes

Then, implement the following methods for `Node`:

- `new`, which creates a new `Node` with the given value and the given vector of adjacents

- `get_value`, which returns a reference to the value of the node

Implement `Debug` for `Node`, so that it prints the value of the node and the values of its

adjacents.

For example, if the node has value `1` and its adjacents are `2` and `3`, it should print:

>[value: 1, adjacents: [2, 3]]

Then, implement the following methods for `Graph`:

- `new`, which creates a `Graph` from a vector of nodes, with the respective adjacents set

- `dfs`, which performs a depth-first search on the graph, starting from the given node. It

returns a vector of nodes, in the order in which they were visited.
 */

/*
#[derive(Eq, PartialEq, Hash)]
struct Node<T: std::hash::Hash + PartialEq + Eq> {
    e: T,
    adj: Vec<Rc<Node<T>>>,
}

struct Graph<T: std::hash::Hash + PartialEq + Eq> {
    nodes: Vec<Rc<Node<T>>>,
}

impl<T: std::hash::Hash + PartialEq + Eq> Node<T> {
    fn new(e: T) -> Self {
        Self { e, adj: Vec::new() }
    }

    fn get_value(&self) -> &T {
        &self.e
    }
}
 
impl<T: std::hash::Hash + PartialEq + Eq + Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[value: {:?}, adjacents: {:?}]", self.e, self.adj.iter().map(|n| n.get_value()).collect::<Vec<&T>>())
    }
}

impl<T: std::hash::Hash + PartialEq + Eq> Graph<T> {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn dfs(&self, n: Rc<Node<T>>) -> Vec<Rc<Node<T>>> {
        let mut h = HashSet::new();
        let mut d = VecDeque::new();
        let mut ret = Vec::new();

        d.push_back(n.clone());
        while d.len() != 0 {
            let out = d.pop_back().unwrap();
            if h.insert(out.clone()) {
                ret.push(out.clone());
                for n1 in out.adj.iter() {
                    d.push_back(n1.clone());
                }
            }
        }
        ret
    }
}
*/
fn main() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
    debug_assert_eq!(list.size, 3);
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 3);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 1);
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    debug_assert_eq!(list.pop(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop(), None);
    list.print_list();
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    debug_assert_eq!(list.tail, None);
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    debug_assert_eq!(list.size, 3);
    println!("{}", list.size);
    list.print_list();
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 1);
    println!("{}", list.head.clone().unwrap().borrow().element);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 3);
    println!("{}", list.tail.clone().unwrap().borrow().element);
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop_back(), None);
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    println!("{:?}", list.head);
    debug_assert_eq!(list.tail, None);
    println!("{:?}", list.tail);
}
