use std::fmt;
use std::ops;

use super::Vector;

extern crate overload;
use overload::overload;
use wasm_bindgen::prelude::*;

/// An nxm Matrix
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix {
    pub m: usize, // # rows
    pub n: usize, // # cols
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Matrix {
    #[wasm_bindgen(constructor)]
    /// Create an empty matrix with m rows and n columns
    pub fn new(m: usize, n: usize) -> Matrix {
        Matrix {m, n, data: vec![0.0; m * n]}
    }

    /// Create a matrix with m rows and n columns from data length m*n
    pub fn from(m: usize, n: usize, data: &[f64]) -> Matrix {
        if m*n != data.len() {
            panic!("Wrong amout of data to make a {}x{} matrix", m, n);
        }
        Matrix {m, n, data: data.to_vec()}
    }

    /// Get value at row i and column j
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self[[i, j]]
    }

    /// Set value at row i and colum j
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self[[i, j]] = v;
    }

    /// Get matrix with select
    pub fn get_rows(&self, r: &[usize]) -> Matrix {
        let mut data = Vec::with_capacity(r.len() * self.n);
        for i in r {
            for j in 0..self.n {
                let idx = i * self.n + j;
                data.push(self.data[idx]);
            }
        }
        Matrix::from(r.len(), self.n, &data)
    }

    /// Get matrix with select columns
    pub fn get_columns(&self, c: &[usize]) -> Matrix {
        let mut data = Vec::with_capacity(self.m * c.len());
        for i in 0..self.m {    
            for j in c {
                let idx = i * self.n + j;
                data.push(self.data[idx]);
            }
        }
        Matrix::from(self.m, c.len(), &data)
    }

    /// Get matrix with select rows and columns
    pub fn get_rows_columns(&self, r: &[usize], c: &[usize]) -> Matrix {
        let mut data = Vec::with_capacity(r.len() * c.len());
        for i in r {
            for j in c {
                let idx = i * self.n + j;
                data.push(self.data[idx]);
            }
        }
        Matrix::from(r.len(), c.len(), &data)
    }

    pub fn swap_rows(&mut self, o: usize, p: usize) {
        let mut temp = Vector::new(self.n);
        for j in 0..self.n {
            temp[j] = self[[o, j]];
            self[[o, j]] = self[[p, j]];
        }
        for j in 0..self.n {
            self[[p, j]] = temp[j];
        }
    }

    pub fn swap_columns(&mut self, o: usize, p: usize) {
        let mut temp = Vector::new(self.m);
        for i in 0..self.m {
            temp[i] = self[[i, o]];
            self[[i, o]] = self[[i, p]];
        }
        for i in 0..self.m {
            self[[i, p]] = temp[i];
        }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        self + other
    }

    pub fn sub(&self, other: &Matrix) -> Matrix {
        self - other
    }

    pub fn mul_vec(&self, other: &Vector) -> Vector {
        self * other
    }

    pub fn mul_mat(&self, other: &Matrix) -> Matrix {
        self * other
    }

    fn gaussian_elimination(&mut self, b: &mut Vector) {
        for i in 0..(self.n-1) { // Rows

            // Partial Pivot
            let mut max_val = self[[i, i]].abs();
            let mut max_idx = i;
            for ii in (i+1)..self.n {
                if self[[ii, i]].abs() > max_val {
                    max_val = self[[ii, i]].abs();
                    max_idx = ii;
                }
            }
            if max_idx != i {
                self.swap_rows(i, max_idx);
                b.swap(i, max_idx);
            }

            // Reduce
            for j in (i+1)..self.n {
                let m = self[[j, i]] / self[[i, i]]; 
                for k in i..self.n {
                    self[[j, k]] -= m * self[[i, k]];
                }
                b[j] -= m * b[i];
            }
        }
    }

    fn back_substitution(&self, y: &Vector) -> Vector {
        let mut x = Vector::new(self.n);
        for i in (0..=(self.n-1)).rev() {
            x[i] = y[i];
            for j in (i+1)..=(self.n-1) {
                x[i] -= self[[i, j]] * x[j];
            }
            x[i] /= self[[i, i]];
        }
        x
    }
    
    /// Solves Ax=b for x using Gaussian Elimination.
    pub fn solve(&self, b: &Vector) -> Vector {
        if self.n != self.m {
            panic!("Matrix must be square to solve");
        }
        if self.n != b.n {
            panic!("Matrix and vector must be same size");
        }
        let mut a_new = self.clone();
        let mut b_new = b.clone();
        a_new.gaussian_elimination(&mut b_new);
        a_new.back_substitution(&b_new)
    }

    /// Solves Ax=b for x using Gaussian Elimination. Mutates self and the given b vector.
    pub fn solve_mut(&mut self, b: &mut Vector) -> Vector {
        if self.n != self.m {
            panic!("Matrix must be square to solve");
        }
        if self.n != b.n {
            panic!("Matrix and vector must be same size");
        }
        self.gaussian_elimination(b);
        self.back_substitution(b)
    }

    pub fn to_string(&self) -> String {
        const PRECISION: usize = 3;
        let mut s = String::new();

        let mut max_len = 0;
        for i in 0..(self.m * self.n) {
            let len = format!("{1:.0$}", PRECISION, self.data[i]).len();
            if len > max_len {
                max_len = len;
            }
        }

        for i in 0..self.m {
            s.push_str("|");
            for j in 0..self.n {
                s.push_str(format!("{num:>0$.1$}", max_len+1, PRECISION, num=self[[i, j]]).as_str());
            }
            s.push_str("|\n");
        }
        s
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


overload!((a: ?Matrix) + (b: ?Matrix) -> Matrix {
    if a.m != b.m || a.n != b.n {
        panic!("Unable to add matrices");
    }
    let mut mat = Matrix::new(a.m, a.n);

    for i in 0..a.m {
        for j in 0..a.n {
            mat[[i, j]] = a[[i, j]] + b[[i, j]];
        }
    }

    mat
});
overload!((a: ?Matrix) - (b: ?Matrix) -> Matrix {
    if a.m != b.m || a.n != b.n {
        panic!("Unable to subtract matrices");
    }
    let mut mat = Matrix::new(a.m, a.n);

    for i in 0..a.m {
        for j in 0..a.n {
            mat[[i, j]] = a[[i, j]] - b[[i, j]];
        }
    }

    mat
});
overload!((a: ?Matrix) * (b: ?Matrix) -> Matrix {
    if a.n != b.m {
        panic!("Can not multiply {}x{} to {}x{}", a.m, a.n, b.m, b.n);
    }
    let mut mat = Matrix::new(a.m, b.n);

    for i in 0..a.m {
        for j in 0..b.n {
            let mut sum = 0.0;
            for k in 0..a.n {
                sum += a[[i, k]] * b[[k, j]];
            }
            mat[[i, j]] = sum;
        }
    }
    mat
});
overload!((a: ?Matrix) * (b: ?Vector) -> Vector {
    if a.n != b.n {
        panic!("Can not multiply {}x{} to {}", a.m, a.n, b.n);
    }
    let mut vec = Vector::new(a.n);

    for i in 0..a.m {
        let mut sum = 0.0;
        for k in 0..a.n {
            sum += a[[i, k]] * b[k];
        }
        vec[i] = sum;
    }
    vec
});


impl ops::Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, idx: [usize; 2]) -> &f64 {
        &self.data[idx[0] * self.n + idx[1]]
    }
}
impl ops::IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut f64 {
        &mut self.data[idx[0] * self.n + idx[1]]
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [f64; 16] = [
        1.0, 2.0, 1.0, -1.0,
        3.0, 2.0, 4.0, 4.0, 
        4.0, 4.0, 3.0, 4.0, 
        2.0, 0.0, 1.0, 5.0
    ];

    #[test]
    fn test_ops() {
        let m0 = Matrix::from(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let m1 = Matrix::from(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(&m0 + &m1, Matrix::from(2, 2, &[2.0, 4.0, 6.0, 8.0]));
        assert_eq!(&m0 - &m1, Matrix::from(2, 2, &[0.0, 0.0, 0.0, 0.0]));

        let m2 = Matrix::from(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let m3 = Matrix::from(3, 2, &[6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(&m2 * &m3, Matrix::from(2, 2, &[20.0, 14.0, 56.0, 41.0]));

        let m4 = Matrix::from(4, 4, &DATA);
        let v0 = Vector::from(&[2.0, 3.0, 1.0, 5.0]);
        assert_eq!(&m4 * &v0, Vector::from(&[4.0, 36.0, 43.0, 30.0]));
    }

    #[test]
    fn test_get() {
        let mat = Matrix::from(4, 4, &DATA);
        assert_eq!(mat.get(0, 0), 1.0);
        assert_eq!(mat.get(1, 1), 2.0);
        assert_eq!(mat.get(0, 1), 2.0);
        assert_eq!(mat.get(1, 0), 3.0);
        assert_eq!(mat.get(2, 0), 4.0);
        assert_eq!(mat.get(3, 2), 1.0);
    }

    #[test]
    fn test_set() {
        let mut mat = Matrix::from(4, 4, &DATA);
        mat.set(0, 0, 5.5);
        assert_eq!(mat.get(0, 0), 5.5);
        mat.set(1, 2, 8.0);
        assert_eq!(mat.get(1, 2), 8.0);
        mat.set(3, 2, 1.2);
        assert_eq!(mat.get(3, 2), 1.2);
    }

    #[test]
    fn test_get_rows() {
        let mat = Matrix::from(4, 4, &DATA);
        let from_rows = mat.get_rows(&[0, 2]);
        
        assert_eq!(from_rows, Matrix::from(2, 4, &[1.0, 2.0, 1.0, -1.0, 4.0, 4.0, 3.0, 4.0,]));
    }

    #[test]
    fn test_get_columns() {
        let mat = Matrix::from(4, 4, &DATA);
        let from_cols = mat.get_columns(&[0, 2]);

        assert_eq!(from_cols, Matrix::from(4, 2, &[1.0, 1.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0]));
    }

    #[test]
    fn test_get_rows_columns() {
        let mat = Matrix::from(4, 4, &DATA);
        let from_rc = mat.get_rows_columns(&[0, 2], &[0, 2]);

        assert_eq!(from_rc, Matrix::from(2, 2, &[1.0, 1.0, 4.0, 3.0]));
    }

    #[test]
    fn test_swap() {
        let mut mat = Matrix::from(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        mat.swap_rows(0, 1);
        mat.swap_columns(0, 2);

        assert_eq!(mat, Matrix::from(2, 3, &[6.0, 5.0, 4.0, 3.0, 2.0, 1.0]));
    }

    #[test]
    fn test_solve() {
        let mat = Matrix::from(4, 4, &DATA);
        let b = Vector::from(&[5.0, 16.0, 22.0, 15.0]);
        let x = mat.solve(&b);

        assert_eq!(x.get(0), 16.0);
        assert_eq!(x.get(1), -6.0);
        assert_eq!(x.get(2), -2.0);
        assert_eq!(x.get(3), -3.0);
    }
}