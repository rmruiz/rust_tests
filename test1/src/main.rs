
#![allow(unused_imports)]
#![allow(dead_code)]
extern crate rand;
use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;

// Define the Node struct
#[allow(unused)]
struct Node {
    value: i32,
    fruit: String,
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
            println!("Value: {}, Fruit: {}", node.value, node.fruit);
            current = &node.next;
        }
    }

    fn push(&mut self, value: i32, fruit: String) {
        let old_head: Option<Box<Node>> = self.head.take();
        let new_head: Box<Node> = Box::new(Node {
            value,
            fruit,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<Node> {
        match self.head.take(){

            Some(n) => {
                self.head = n.next;
                Some(Node {
                    value: n.value,
                    fruit: n.fruit,
                    next: None,
                })
            },
            None => None,
        }
    }
    
}

#[allow(unused)]
fn main() {
    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
        "Fig".to_string(),
        "Grape".to_string(),
    ];

    let mut rng = rand::thread_rng();
    let mut list: LinkedList = LinkedList::new();

    for fruit in fruits {
            let value: i32 = rng.gen_range(1..101);
            list.push(value, fruit);
        }
    list.print();
    println!("__");
    list.pop();
    list.print();

}
