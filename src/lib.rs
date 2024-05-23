use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BTreeArr {
    tree_vec: btree_vec::BTreeVec<f64>,
}

#[wasm_bindgen]
impl BTreeArr {
    pub fn new() -> BTreeArr {
        BTreeArr {
            tree_vec: btree_vec::BTreeVec::new(),
        }
    }

    pub fn push(&mut self, value: f64) {
        self.tree_vec.push(value);
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        self.tree_vec.get(index).cloned()
    }

    pub fn set(&mut self, index: usize, value: f64) {
        if let Some(val) = self.tree_vec.get_mut(index) {
            *val = value;
        }
    }

    pub fn len(&self) -> usize {
        self.tree_vec.len()
    }

    pub fn remove(&mut self, index: usize) -> Option<f64> {
        if let Some(_) = self.tree_vec.get(index) {
            Some(self.tree_vec.remove(index))
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: usize, value: f64) {
        if index <= self.tree_vec.len() {
            self.tree_vec.insert(index, value);
        }
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.tree_vec.pop()
    }
}
