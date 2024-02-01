use num_bigint::BigInt;

use crate::{algebras::Rings::{classes::PolynomialRing::PolynomialRing, instances::PolynomialRing_instance::PolynomialRingInstance}, numbers::numbers::{ClassInstance, Instance, Number, Operand, StatefulClass}, poly::{classes::univariate_polynomial::UnivariatePolynomial, instances::univariate_polynomial_instance::UnivariatePolynomialInstance}, variables::vars::Var};

use super::matrix::Matrix;


#[derive(Clone, Debug)]
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

impl<T> std::ops::Add<Vector<PolynomialRingInstance<T>>> for Vector<PolynomialRingInstance<T>> where T: Instance + Clone + PartialEq + Operand + Number + ClassInstance + 'static {
    type Output = Vector<PolynomialRingInstance<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.len == rhs.len {
            let mut temp_values = self.values.clone();
            for i in 0..self.len {
                temp_values[i] = temp_values[i].clone().add(rhs.values[i].clone());
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


// generic functions for vectors
impl<T> Vector<T> where T: Instance + Clone + PartialEq + Operand {
    pub fn new(vector: Vec<T>) -> Vector<T> {
        Vector {values: vector.clone(), len: vector.len()}
    }
}


/*
    Vector operations for Univariate Polynomials
*/
impl<T> Vector<UnivariatePolynomialInstance<T>> where T: Instance + Clone + PartialEq + Operand + Number + Eq + ClassInstance + 'static {
   

    pub fn transpose(&self) -> Matrix<UnivariatePolynomialInstance<T>> {
        let rows: usize = 1;
        let columns = self.len;

        let mut container: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();

        for i in 0..self.len {
            let temp_vect: Vec<UnivariatePolynomialInstance<T>> = vec![self.values[i].clone()];
            container.push(temp_vect);
        }

        Matrix::new(container, rows, columns)
    }

    pub fn l_1_norm(&self) -> UnivariatePolynomialInstance<T> {
        let generator: Box<dyn StatefulClass> = self.values[0].coefficients[0].get_class();
        let mut sum: UnivariatePolynomialInstance<T> = UnivariatePolynomial::zero(Var::new("x", BigInt::from(1)), &generator);
        for i in 0..self.values.len() {
            sum = sum.add(&self.values[i]);
        }

        sum
    }

    pub fn l_2_norm(&self) -> UnivariatePolynomialInstance<T> {
        let generator: Box<dyn StatefulClass> = self.values[0].coefficients[0].get_class();

        let mut sum: UnivariatePolynomialInstance<T> = UnivariatePolynomial::zero(Var::new("x", BigInt::from(1)), &generator);
        for i in 0..self.values.len() {
            sum = sum.add(&(self.values[i].clone().mul(&self.values[i])));
        }

        sum
    }

    pub fn l_inf_norm(&self) -> UnivariatePolynomialInstance<T> {
        let generator: Box<dyn StatefulClass> = self.values[0].coefficients[0].get_class();

        let mut max: UnivariatePolynomialInstance<T> = UnivariatePolynomial::zero(Var::new("x", BigInt::from(1)), &generator);
        for i in 0..self.values.len() {
            if self.values[i].greater_than(&max) {
                max = self.values[i].clone();
            }
        }

        max
    }

    pub fn element_wise_product(&self, m: Vector<UnivariatePolynomialInstance<T>>) -> UnivariatePolynomialInstance<T> {
        if self.len != m.len {
            panic!("Cannot perform element-wise product with different size vectors");
        }
        let generator: Box<dyn StatefulClass> = self.values[0].coefficients[0].get_class();

        let mut sum: UnivariatePolynomialInstance<T> = UnivariatePolynomial::zero(Var::new("x", BigInt::from(1)), &generator);
        for i in 0..self.len {
            sum = sum.add(&(self.values[i].mul(&m.values[i])));
        }

        sum
    }

}


/*
    Vector operations for Polynomial Rings
*/
impl<T> Vector<PolynomialRingInstance<T>> where T: Instance + Clone + PartialEq + Operand + Number + Eq + ClassInstance + 'static{
   

    pub fn transpose(&self) -> Matrix<PolynomialRingInstance<T>> {
        let rows: usize = 1;
        let columns = self.len;

        let mut container: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();

        for i in 0..self.len {
            let temp_vect: Vec<PolynomialRingInstance<T>> = vec![self.values[i].clone()];
            container.push(temp_vect);
        }

        Matrix::new(container, rows, columns)
    }

    pub fn l_1_norm(&self) -> PolynomialRingInstance<T> {
        let variable: Var = self.values[0].clone().var.clone();
        let generator: PolynomialRing<T> = self.values[0].clone().class.into_inner().clone();
        let generator_values: Box<dyn StatefulClass> = self.values[0].clone().coefficients[0].get_class();

        let mut sum: PolynomialRingInstance<T> = generator.zero(variable, &generator_values);
        for i in 0..self.values.len() {
            sum = sum.add(&self.values[i]);
        }

        sum
    }

    pub fn l_2_norm(&self) -> PolynomialRingInstance<T> {
        let variable: Var = self.values[0].var.clone();
        let generator: PolynomialRing<T> = self.values[0].clone().class.into_inner().clone();
        let generator_values: Box<dyn StatefulClass> = self.values[0].clone().coefficients[0].get_class();

        let mut sum: PolynomialRingInstance<T> = generator.zero(variable, &generator_values);
        for i in 0..self.values.len() {
            sum = sum.add(&(self.values[i].clone().mul(&self.values[i])));
        }

        sum
    }

    pub fn l_inf_norm(&self) -> PolynomialRingInstance<T> {
        let variable: Var = self.values[0].var.clone();
        let generator: PolynomialRing<T> = self.values[0].clone().class.into_inner().clone();
        let generator_values: Box<dyn StatefulClass> = self.values[0].clone().coefficients[0].get_class();

        let mut max: PolynomialRingInstance<T> = generator.zero(variable, &generator_values);
        for i in 0..self.values.len() {
            if self.values[i].greater_than(&max) {
                max = self.values[i].clone();
            }
        }

        max
    }

    pub fn element_wise_product(&self, m: Vector<PolynomialRingInstance<T>>) -> PolynomialRingInstance<T> {
        if self.len != m.len {
            panic!("Cannot perform element-wise product with different size vectors");
        }

        let variable: Var = self.values[0].var.clone();
        let generator: PolynomialRing<T> = self.values[0].clone().class.into_inner().clone();
        let generator_values: Box<dyn StatefulClass> = self.values[0].clone().coefficients[0].get_class();

        let mut sum: PolynomialRingInstance<T> = generator.zero(variable, &generator_values);
        for i in 0..self.len {
            sum = sum.add(&(self.values[i].mul(&m.values[i])));
        }

        sum
    }

}

/*
    Vector operation for generic numbers
*/
impl<T> Vector<T> where T: Instance + Clone + PartialEq + Operand + Number {
    

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