use wasm_bindgen::prelude::*;

/// An nth dimensional vector
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Vector {
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Vector {
        Vector {data: vec![0.0; n]}
    }

    pub fn from(data: &[f64]) -> Vector {
        Vector {data: data.to_vec()}
    }

    pub fn get(&self, i: usize) -> f64 {
        self.data[i]
    }

    pub fn set(&mut self, i: usize, v: f64) {
        self.data[i] = v;
    }
}