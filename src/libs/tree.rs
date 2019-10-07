#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Node {
    value: u32,
    left:  Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn new(value: u32) -> Node {
        Node { value, left: None, right: None }
    }

    pub fn add(&mut self, value: u32) {
        if self.value > value {
            match &mut self.left {
                Some(x) => x.add(value),
                None => self.create_left(value)
            }
        } else {
            match &mut self.right {
                Some(x) => x.add(value),
                None => self.create_right(value)
            }
        }
    }

    pub fn create_left(&mut self, value: u32) {
        self.left = Some(Box::new(Node::new(value)))
    }

    pub fn create_right(&mut self, value: u32) {
        self.right = Some(Box::new(Node::new(value)))
    }

    pub fn show(&self, vec: &mut Vec<u32>) {
        if let Some(left) = &self.left {
            left.show(vec);
        }

        vec.push(self.value);

        if let Some(right) = &self.right {
            right.show(vec);
        }
    }
}

struct Tree {
    root: Option<Box<Node>>
}

impl Tree {
    fn new() -> Tree {
        Tree { root: None }
    }

    pub fn add(&mut self, value: u32) {
        match &mut self.root {
            Some(x) => x.add(value),
            None => self.create(value)
        };
    }

    pub fn create(&mut self, value: u32) {
        self.root = Some(Box::new(Node::new(value)));
    }

    pub fn show(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = vec![];

        if let Some(node) = &self.root {
            node.show(&mut vec);
        }

        vec
    }
}

lazy_static! {
    static ref STATES: Mutex<Tree> = Mutex::new(Tree::new());
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Iniciando App");
}

#[wasm_bindgen]
pub fn add(value: u32) {
    let mut tree = STATES.lock().unwrap();
    tree.add(value);
}

#[wasm_bindgen]
pub fn show() -> Vec<u32> {
    let tree = STATES.lock().unwrap();
    tree.show()
}