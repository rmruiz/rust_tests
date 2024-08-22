#![allow(unused_imports)]
#![allow(dead_code)]
extern crate rand;
use rand::Rng;
use rand::thread_rng;

extern crate rand_distr;
use rand_distr::{Normal, Distribution};

use std::cmp::Ordering;

// Define the Node struct
#[allow(unused)]
struct Node {
    id: i32,
    value: f32,
    next: Option<Box<Node>>,
}

// Define the LinkedList struct
#[allow(unused)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

#[allow(unused)]
impl LinkedList {
    // Create a new empty LinkedList
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn print(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        while let Some(ref node) = current {
            println!("ID: {}, Value: {}", node.id, node.value);
            current = &node.next;
        }
    }

    fn push(&mut self, id: i32, value: f32) {
        let old_head: Option<Box<Node>> = self.head.take();
        let new_head: Box<Node> = Box::new(Node {
            id,
            value,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<Node> {
        match self.head.take() {
            Some(n) => {
                self.head = n.next;
                Some(Node {
                    id: n.id,
                    value: n.value,
                    next: None,
                })
            },
            None => None,
        }
    }
    
}

#[allow(unused)]
fn main() {
 

    let mut rng = rand::thread_rng();
    let mut list: LinkedList = LinkedList::new();

    // mean 2, standard deviation 3
    let normal = Normal::new(4.0, 2.0).unwrap();

    for id in 1..101 {
            let value: f32 = normal.sample(&mut rand::thread_rng());
            list.push(id, value);
        }
    list.print();
    // println!("__");
    // list.pop();
    // list.print();
}
