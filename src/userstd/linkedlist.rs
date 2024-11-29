#![allow(unused)]
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}
#[derive(Debug)]
pub struct LinkedList {
    length: usize,
    head: Option<Rc<RefCell<Node>>>,
}
impl LinkedList {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn push(&mut self, data: i32) {
        let new_node = Node {
            data,
            next: self.head.clone(),
        };
        self.head = Some(Rc::new(RefCell::new(new_node)));
        self.length += 1;
    }
    pub fn get(&mut self) -> Option<i32> {
        if let Some(rc_node) = self.head.take() {
            return Some(rc_node.borrow().data);
        }
        None
    }
    pub fn pop(&mut self) -> Option<i32> {
        if let Some(rc_node) = self.head.take() {
            self.length -= 1;
            self.head = rc_node.borrow().next.clone();
            return Some(rc_node.borrow().data);
        }
        None
    }
}
