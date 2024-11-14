/*
define the Nextable trait with a method gimme_next implement Nextable for i32, 
gimme_next returns the optional successor of self implement Nextable for char, 
gimme_next returns the optional new char that is the next char (as a u32 conversion) 
implement a function printnext that takes a Nextable and prints the argument
and its gimme_next using the ":?" formatter It behaves as the example: next of 5 is Some(6) next of 's' is Some('t')
 */

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

trait Nextable {
    fn gimme_next(&self) -> Option<Self> where Self: Sized; // dfk if they wanted this
}

impl Nextable for i32 {
    fn gimme_next(&self) -> Option<Self> where Self: Sized {
        return Some(self + 1);
    }
}

impl Nextable for char {
    fn gimme_next(&self) -> Option<Self> where Self: Sized {
        return Some(char::from_u32(*self as u32 + 1)?);
    }
}

fn printnext(v: impl Nextable + std::fmt::Debug) {
    let tmp = v.gimme_next();
    println!("next of {:?} is {:?}", v, tmp)
}

/* */

/*
- Write a struct `ConsIter` that has a field `iter` of type `Chars` (`std::str::Chars`)

- Write a struct `Wrapper` that has a field `inner` of type `String`, write a method `iter` for `Wrapper` that returns a `ConsIter`

- Implement `Iterator` for `ConsIter` that iterates over `chars`, it yields all the characters that are part of the ascii code and aren't vocals ("aeiou")

Hints: use `is_ascii()` to check if a char is actually ascii, use `to_ascii_lowercase()` for managing mixed-cased words.

 */

struct ConsIter<'a> {
    iter: std::str::Chars<'a>,
}

struct Wrapper {
    inner: String,
}

impl Wrapper {
    fn iter(&self) -> ConsIter {
        ConsIter { iter: self.inner.chars() }
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.iter.next() {
            if c.is_ascii() && !is_vowel(c.to_ascii_lowercase()) {
                return Some(c);
            }
        }
        None
    }
}

/* */

/*
 write a function basicbox_sum that takes a vector of Strings and returns a vector of 
 Boxes of usizes the returned vector contains all the lengths of the input vector followed 
 by a final element that sums all the previous lengths
 */

fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut ret: Vec<Box<usize>> = v.iter().map(|s| Box::new(s.len())).collect();
    ret.push(Box::new(ret.iter().fold(0, |acc: usize, b: &Box<usize>| acc + **b)));
    ret
}

/* */

/*
Write a two structs: `MasterClock` and `Slave clock` that both derive Debug.

`MasterClock` keeps track of a number of clock cycle (in usize).

The struct has:

-[1] a `new()` method that initialize it with clock at zero.

-[1] a `tick(&mut self)` method that increase the clock cycle by 1.

-[2] a `get_slave(&sef)` method that return an object of type `SlaveClock`

`SlaveClock` can be built only using the `MasterClock.get_slave(&self)` method, and has a method named  [2]`get_clock(&self)` that returns the current clock (that automatically sinks with the master clock)  

If you found this description confusing, please read the test cases, and it will become clearer.
 */

 #[derive(Debug)]
struct MasterClock {
    tick: Rc<RefCell<usize>>,
}

#[derive(Debug)]
struct SlaveClock {
    tick: Rc<RefCell<usize>>,
}

impl MasterClock {
    fn new() -> Self {
        MasterClock { tick: Rc::new(RefCell::new(0)) }
    }

    fn tick(&mut self) {
        *self.tick.borrow_mut() += 1;
    }

    fn get_slave(&self) -> SlaveClock {
        SlaveClock { tick: self.tick.clone() }
    }
}

impl SlaveClock {
    fn get_clock(&self) -> usize {
        *self.tick.borrow()
    }
}

/* */

/* 
 define a module "finance". inside it, define two public modules "wallet_1" and "wallet_2".

[1] define a struct "Wallet" inside "wallet_1" with an attribute "euro" with type f32.

[1] define a struct "Wallet" inside "wallet_2" with an attribute "euro" with type u32, and an attribute "cents" with type u8

derive Debug on both "Wallet", and make all attributes public

create two public alias inside "finance":

- Wallet1 for wallet_1::Wallet

- Wallet2 for wallet_2::Wallet

[2] define a public function "compare_wallet" in the module "finance" that takes two arguments: "first" with type "&Wallet1" and "second" with type "&Wallet2" the function returns true if "first" has more money that "second", otherwise it returns false

 */

mod finance {
    pub type Wallet1 = wallet_1::Wallet;
    pub type Wallet2 = wallet_2::Wallet;

    pub mod wallet_1 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro: f32,
        }
    }

    pub mod wallet_2 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro: u32,
            pub cents: u8,
        }
    }

    pub fn compare_wallet(w1: &Wallet1, w2: &Wallet2) -> bool {
        return w1.euro > (w2.euro as f32 + w2.cents as f32 / 100.)
    }
}

/* */

/*
take the following `Tree`, `Node`, and `Content` structs define these functions / methods for `Tree`
 - new [1] : creates an empty tree
 - add [6]: takes a generic element `el` and adds a node to the tree whose content is `el` and such that
 nodes on the left have contents which are < smaller than the current node,
 nodes on the center have contents which are == to the current node,
 nodes on the right have contents which are > than the current node
- howmany_smaller [4] : takes a generic element `el` and returns an i32 telling how many nodes
    does the tree have that are < than `el` 
- implement `PartialOrd` for `Content` [4]: contents can be compared by comparing the `len` of
    their String fields

Note: the tests already include the code below, all you need to paste as the answer are the `impl` blocks
and possible imports (use ...).
 */



#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub s:String
}

impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.s.len().partial_cmp(&other.s.len())
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}
impl <T>Node<T>{
    pub fn new(elem:T) -> Node<T> {
        Node{elem,left:None,center:None,right:None}
    }
    }
#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
}
type TreeLink<T> = Option<Box<Node<T>>>;

impl<T: PartialOrd> Node<T> {
    // this could be optimized but idk if there's enough time at the exam so
    fn howmany_smaller(&self, e: &T) -> i32 {
        let e1: i32 = if self.elem < *e { 1 } else { 0 };
        let mut e2: i32 = 0;
        let mut e3: i32 = 0;
        let mut e4: i32 = 0;
        if self.left.is_some() {
            e2 = self.left.as_ref().unwrap().howmany_smaller(e);
        }
        if self.center.is_some() {
            e3 = self.center.as_ref().unwrap().howmany_smaller(e);
        }
        if self.right.is_some() {
            e4 = self.right.as_ref().unwrap().howmany_smaller(e);
        }
        e1 + e2 + e3 + e4
    }
}

impl<T: PartialOrd> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn add(&mut self, e: T) {
        let mut tmp = &mut self.root;
        while let Some(n) = tmp {
            if n.elem < e {
                tmp = &mut n.right;
            }
            else if n.elem == e {
                tmp = &mut n.center;
            }
            else {
                tmp = &mut n.left;
            }
        }
        *tmp = Some(Box::new(Node::new(e)));
    }

    fn howmany_smaller(&self, e: &T) -> i32 {
        match self.root {
            None => 0,
            Some(ref n) => n.howmany_smaller(e)
        }
    }
}

fn main() {
    let t: Tree<i32> = Tree::new();
    println!("{:?}",t);

    let e0 = Content::new(10,"asd".to_string());
    let e1 = Content::new(10,"who".to_string());
    let e2 = Content::new(11,"oneasd".to_string());
    println!("{}", e1==e2);
    println!("{}", e1==e0);
    println!("{}", e1<e2);
    println!("{}", e1>e2);

    let mut t = Tree::new();
    let e1 = 10;
    let e2 = 11;
    let e3 = 8;
    let e4 = 19;
    let e5 = 45;
    t.add(e1); t.add(e2); t.add(e3); t.add(e4); t.add(e5);
    println!("{:?}",t);

    let t = Tree{
    root: Some(Box::new(Node{
    elem: 10, //Content::new(10,"what".to_string()),
    left: Some(Box::new(Node{
    elem: 11, //Content::new(11,"who".to_string()),
    left: None,
    center: Some(Box::new(Node{
    elem: 19, //Content::new(19,"ten".to_string()),
    left: None,
    center: None,
    right: None
    })),
    right: None
    })),
    center: Some(Box::new(Node{
    elem: 15, //Content::new(15,"zips".to_string()),
    left: None,
    center: None,
    right: None
    })),
    right: Some(Box::new(Node{
    elem: 88, //Content::new(88,"whose".to_string()),
    left: None,
    center: None,
    right: None
    }))
    }))
    };
    let e56 = 4; //Content::new(45,"gips".to_string());
    let e57 = 40; //Content::new(45,"naasdasdsad".to_string());
    println!("{}", t.howmany_smaller(&e56));
    println!("{}", t.howmany_smaller(&e57));
}
