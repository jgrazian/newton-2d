use std::fmt;
use std::ops;

extern crate overload;
use overload::overload;
use wasm_bindgen::prelude::*;

/// An nth dimensional vector
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Vector {
    pub n: usize,
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(n: usize) -> Vector {
        Vector {n, data: vec![0.0; n]}
    }

    pub fn from(data: &[f64]) -> Vector {
        Vector {n: data.len(), data: data.to_vec()}
    }

    pub fn get(&self, i: usize) -> f64 {
        self.data[i]
    }

    pub fn set(&mut self, i: usize, v: f64) {
        self.data[i] = v;
    }

    pub fn swap(&mut self, o: usize, p: usize) {
        let tmp = self[o];
        self[o] = self[p];
        self[p] = tmp;
    }

    pub fn dot(a: &Vector, b: &Vector) -> f64 {
        let mut dot = 0.0;
        for i in 0..a.n {
            dot += a[i] * b[i];
        }
        dot
    }

    pub fn to_string(&self) -> String {
        const PRECISION: usize = 3;
        let mut s = String::new();

        let mut max_len = 0;
        for i in 0..self.n {
            let len = format!("{1:.0$}", PRECISION, self.data[i]).len();
            if len > max_len {
                max_len = len;
            }
        }

        for i in 0..self.n {
            s.push_str(format!("|{num:>0$.1$} |\n", max_len+1, PRECISION, num=self.data[i]).as_str());
        }
        s
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

overload!((a: ?Vector) + (b: ?Vector) -> Vector {
    if a.n != b.n {
        panic!("Unable to add vectors");
    }
    let mut v = Vector::new(a.n);

    for i in 0..a.n {
        v[i] = a[i] + b[i];
    }

    v
});
overload!((a: ?Vector) - (b: ?Vector) -> Vector {
    if a.n != b.n {
        panic!("Unable to subtract vectors");
    }
    let mut v = Vector::new(a.n);

    for i in 0..a.n {
        v[i] = a[i] - b[i];
    }

    v
});
overload!((a: ?Vector) * (b: ?Vector) -> Vector {
    if a.n != b.n {
        panic!("Unable to multiply vectors of differing lengths");
    }
    let mut v = Vector::new(a.n);
    for i in 0..a.n {
        v[i] = a[i] * b[i];
    }
    v
});

impl ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.data[idx]
    }
}
impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        &mut self.data[idx]
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [f64; 4] = [1.0, 3.0, 4.0, 2.0];

    #[test]
    fn test_get() {
        let vec = Vector::from(&DATA);
        assert_eq!(vec.get(0), 1.0);
        assert_eq!(vec.get(1), 3.0);
        assert_eq!(vec.get(2), 4.0);
        assert_eq!(vec.get(3), 2.0);
    }

    #[test]
    fn test_set() {
        let mut vec = Vector::from(&DATA);
        vec.set(1, 8.0);
        assert_eq!(vec.get(1), 8.0);
        vec.set(3, 11.0);
        assert_eq!(vec.get(3), 11.0);
    }

}
