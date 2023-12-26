use crate::numbers::numbers::{Instance, Operand, Number};

use super::matrix::Matrix;


#[derive(Clone)]
pub struct Vector<T> {
    pub values: Vec<T>,
    pub len: usize
}


impl<T> std::ops::Add for Vector<T> where T: Instance + Clone + PartialEq + Operand + Number {
    type Output = Vector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.len == rhs.len {
            let mut temp_values = self.values.clone();
            for i in 0..self.len {
                temp_values[i] = temp_values[i].add(&rhs.values[i]);
            }

            Vector::new(temp_values)
        } else {
            panic!("Cannot sum different size vectors");
        }
    }
    
}

impl<T> std::ops::Sub for Vector<T> where T: Instance + Clone + PartialEq + Operand + Number {
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.len == rhs.len {
            let mut temp_values = self.values.clone();
            for i in 0..self.len {
                temp_values[i] = temp_values[i].sub(&rhs.values[i]);
            }

            Vector::new(temp_values)
        } else {
            panic!("Cannot sum different size vectors");
        }
    } 
}




impl<T> Vector<T> where T: Instance + Clone + PartialEq + Operand + Number {
    pub fn new(vector: Vec<T>) -> Vector<T> {
        Vector {values: vector.clone(), len: vector.len()}
    }

    pub fn transpose(&self) -> Matrix<T> {
        let rows: usize = 1;
        let columns = self.len;

        let mut container: Vec<Vec<T>> = Vec::new();

        for i in 0..self.len {
            let temp_vect: Vec<T> = vec![self.values[i].clone()];
            container.push(temp_vect);
        }

        Matrix::new(container, rows, columns)
    }

    pub fn l_1_norm(&self) -> T {
        let mut sum: T = T::zero();
        for i in 0..self.values.len() {
            sum = sum.add(&self.values[i]);
        }

        sum
    }

    pub fn l_2_norm(&self) -> T {
        let mut sum: T = T::zero();
        for i in 0..self.values.len() {
            sum = sum.add(&(self.values[i].clone().mul(&self.values[i])));
        }

        sum
    }

    pub fn l_inf_norm(&self) -> T {
        let mut max: T = T::zero();
        for i in 0..self.values.len() {
            if self.values[i].greater_than(&max) {
                max = self.values[i].clone();
            }
        }

        max
    }

    pub fn element_wise_product(&self, m: Vector<T>) -> T {
        if self.len != m.len {
            panic!("Cannot perform element-wise product with different size vectors");
        }
        let mut sum: T = T::zero();
        for i in 0..self.len {
            sum = sum.add(&(self.values[i].mul(&m.values[i])));
        }

        sum
    }

}