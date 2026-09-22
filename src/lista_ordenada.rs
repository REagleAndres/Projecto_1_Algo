use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct ListaOrdenada { 
    vector: Vec<i32>,
}
impl ListaOrdenada {

    pub fn Init() -> Self{
        Self { vector: Vec::new() }
    }

    pub fn Done(self) {}

    pub fn Clear(&mut self) {
        self.vector.clear();
    }

    pub fn Insert(&mut self, element: i32){
        self.vector.append();
        self.vector.sort();
    }
}

