use wasm_bindgen::prelude::*;
use super::Vector;

/// An nxm Matrix
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix {
    m: usize,
    n: usize,
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Matrix {
    #[wasm_bindgen(constructor)]
    pub fn new(m: usize, n: usize) -> Matrix {
        Matrix {m, n, data: vec![0.0; m * n]}
    }

    pub fn from(m: usize, n: usize, data: &[f64]) -> Matrix {
        if m*n != data.len() {
            panic!("Bad matrix");
        }
        Matrix {m, n, data: data.to_vec()}
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.m + j]
    }

    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.data[i * self.m + j] = v;
    }

    pub fn gaussian_elimination(&mut self, b: &mut Vector) {
        for j in 0..(self.n-1) {
            for i in (j+1)..self.n {
                let m = self.get(i, j) / self.get(j, j);
                for k in j..self.n {
                    self.set(i, k, self.get(i, k) - m*self.get(j, k));
                }
                b.set(i, b.get(i) - m*b.get(j));
            }
        }
    }

    pub fn back_substitution(&self, y: &Vector) -> Vector {
        let mut x = Vector::new(self.n);
        for i in (0..=(self.n-1)).rev() {
            x.set(i, y.get(i));
            for j in (i+1)..=(self.n-1) {
                x.set(i, x.get(i) - self.get(i, j)*x.get(j));
            }
            x.set(i, x.get(i) / self.get(i, i));
        }
        x
    }

    pub fn solve(&mut self, b: &Vector) -> Vector {
        let mut b_new = b.clone();
        self.gaussian_elimination(&mut b_new);
        self.back_substitution(&b_new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = [1.0, 2.0, 1.0, -1.0,
                     3.0, 2.0, 4.0, 4.0, 
                     4.0, 4.0, 3.0, 4.0, 
                     2.0, 0.0, 1.0, 5.0
                    ];
        let mut m = Matrix::from(4, 4, &data);
        let b = Vector::from(&[5.0, 16.0, 22.0, 15.0]);
        let x = m.solve(& b);

        assert_eq!(x.get(0), 16.0);
        assert_eq!(x.get(1), -6.0);
        assert_eq!(x.get(2), -2.0);
        assert_eq!(x.get(3), -3.0);
    }
}