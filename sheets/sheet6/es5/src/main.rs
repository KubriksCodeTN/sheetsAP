#![allow(dead_code)]

use std::marker::PhantomData;
trait CompileTimeNode{
    type LeftType: CompileTimeNode;
    type RightType: CompileTimeNode;
    fn is_none() -> bool;   
}

struct NullNode{}

struct Node<L,R>{
    left: PhantomData<L>,
    right: PhantomData<R>,   
}

impl CompileTimeNode for NullNode {
    type LeftType = NullNode;
    type RightType = NullNode;
    fn is_none() -> bool {
        true
    }
}

impl<L: CompileTimeNode, R: CompileTimeNode> CompileTimeNode for Node<L, R> {
    type LeftType = L;
    type RightType = R;
    fn is_none() -> bool {
        false
    }
}

fn count_nodes<T: CompileTimeNode>() -> usize {
    return if !T::is_none() { 1 + count_nodes::<T::LeftType>() + count_nodes::<T::RightType>() } 
           else { 0 }
}

#[test]
fn test(){
    let len = count_nodes::<
    Node<
    Node<
    Node<
    NullNode,
    NullNode,
    >,NullNode
    >,
    Node<
    Node<
    Node<
    Node<
    NullNode,
    NullNode
    >,
    NullNode
    >,
    Node<
    NullNode,
    NullNode
    >
    >,
    NullNode
    >
    >
    >();
    assert_eq!(len, 8);
}   

fn main() {
    println!("Hello, world!");
}
