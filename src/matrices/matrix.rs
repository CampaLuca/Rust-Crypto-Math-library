use std::fmt::Display;

use either::Either;
use either::Either::Right;
use either::Left;
use num_bigint::BigInt;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::numbers::{Class, ClassInstance, Instance, Number, Operand, StatefulClass};
use crate::matrices::vector::Vector;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use crate::variables::vars::Var;

/*
Notes on the implementation

Matrices are composed by vectors. Then, the code will represent matrices as a list of vectors. 
Each vector is represented as a list. Vectors MUST be seen as vertical vectors, NOT horizontal vectors.
Then you should think to work as columns x rows
*/
// INSTANCES
#[derive(Clone)]
pub struct Matrix<T> {
    pub values: Vec<Vec<T>>,
    pub rows: usize,
    pub columns: usize
}


impl<T> PartialEq for Matrix<T> where T: Instance + Clone + PartialEq + Operand{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.columns == other.columns && self.values == other.values
    }
}
impl<T> Eq for Matrix<T> where T: Instance + Clone + Eq + Operand{}


// generic functions for matrices
impl<T> Matrix<T> where T: Instance + Clone + PartialEq {
    pub fn new(values: Vec<Vec<T>>, rows: usize, columns: usize) -> Matrix<T> {
        Matrix { values: values, rows: rows, columns: columns}
    }
}



// operations
impl<T> std::ops::Add for Matrix<T> where T: Instance + Clone + Operand + PartialEq {
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.columns == rhs.columns && self.rows == rhs.rows {
            let mut result = self.clone();
            for i in 0..self.columns {
                for j in 0..self.rows {
                    result.values[i][j] = self.values[i][j].add(&rhs.values[i][j]);
                }
            }

            result
        } else {
            panic!("Cannot add matrix with different sizes");
        }
    }
}

impl<T> std::ops::Sub for Matrix<T> where T: Instance + Clone + Operand + PartialEq {
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.columns == rhs.columns && self.rows == rhs.rows {
            let mut result = self.clone();
            for i in 0..self.columns {
                for j in 0..self.rows {
                    result.values[i][j] = self.values[i][j].sub(&rhs.values[i][j]);
                }
            }

            result
        } else {
            panic!("Cannot subtract matrix with different sizes");
        }
    }
}


impl<T> std::ops::Mul for Matrix<PolynomialRingInstance<T>> where T: Display + 'static + Instance + Clone + Operand + ClassInstance + PartialEq + Number {
    type Output = Either<Matrix<PolynomialRingInstance<T>>, Vector<PolynomialRingInstance<T>>>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns == rhs.rows {
            let generator: PolynomialRing<T> = self.values[0][0].clone().class.into_inner().clone();
            let generator_values: Box<dyn StatefulClass> = self.values[0][0].clone().coefficients[0].get_class();

            let var: Var = self.values[0][0].var.clone();
            let mut result_vectors: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();
            
            for j in 0..rhs.columns {
                let mut temp_vector: Vec<PolynomialRingInstance<T>> = Vec::new();
                for i in 0..self.rows {
                    
                    let mut accumulator: PolynomialRingInstance<T> = generator.clone().zero(var.clone(), &generator_values);

                    for old_columns in 0..self.columns {
                        for old_rows in 0..rhs.rows {
                            accumulator = accumulator.add(&(self.values[old_columns][i].clone().mul(rhs.values[j][old_rows].clone())));
                        }
                    }

                    temp_vector.push(accumulator);
                }

                result_vectors.push(temp_vector);
            }

            if rhs.columns == 1 {
                return Right(Vector::new(result_vectors[0].clone()));
            } else {
                return Left(Matrix::new(result_vectors, self.rows, rhs.columns));
            }
        } else {
            panic!("Cannot multiplicate those matrices");
        }
    }
}

impl<T> std::ops::Mul for Matrix<T> where T: Instance +  Clone + Operand + PartialEq + Number {
    type Output = Either<Matrix<T>, Vector<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.columns == rhs.rows {
            let mut result_vectors: Vec<Vec<T>> = Vec::new();
            
            for j in 0..rhs.columns {
                let mut temp_vector: Vec<T> = Vec::new();
                for i in 0..self.rows {
                    
                    let mut accumulator: T = T::zero();

                    for old_columns in 0..self.columns {
                        for old_rows in 0..rhs.rows {
                            accumulator = accumulator.add(&(self.values[old_columns][i].mul(&rhs.values[j][old_rows])));
                        }
                    }

                    temp_vector.push(accumulator);
                }

                result_vectors.push(temp_vector);
            }

            if rhs.columns == 1 {
                return Right(Vector::new(result_vectors[0].clone()));
            } else {
                return Left(Matrix::new(result_vectors, self.rows, rhs.columns));
            }
        } else {
            panic!("Cannot multiplicate those matrices");
        }
    }
}


impl<T> std::ops::Mul<Vector<PolynomialRingInstance<T>>> for Matrix<PolynomialRingInstance<T>> where T: Display + 'static + ClassInstance + Instance + Clone + Operand + PartialEq + Number {
    type Output = Vector<PolynomialRingInstance<T>>;
    fn mul(self, rhs: Vector<PolynomialRingInstance<T>>) -> Self::Output {
        if self.columns == rhs.len {
            let variable: Var = self.values[0][0].var.clone();
            let generator: PolynomialRing<T> = self.values[0][0].clone().class.into_inner().clone();
            let generator_values: Box<dyn StatefulClass> = self.values[0][0].clone().coefficients[0].get_class();

            let mut temp_vector: Vec<PolynomialRingInstance<T>> = Vec::new();
            for i in 0..self.rows {
                let mut accumulator: PolynomialRingInstance<T> = generator.clone().zero(variable.clone(), &generator_values);

                for old_columns in 0..self.columns {
                    for old_rows in 0..rhs.len {
                        accumulator = accumulator.add(&(self.values[old_columns][i].clone().mul(rhs.values[old_rows].clone())));
                    }
                }

                temp_vector.push(accumulator);
            }

            Vector::new(temp_vector)
            
        } else {
            panic!("Cannot multiplicate those matrices");
        }
    }
}
impl<T> std::ops::Mul<Vector<T>> for Matrix<T> where T: Instance + Clone + Operand + PartialEq + Number {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        if self.columns == rhs.len {
                 
            let mut temp_vector: Vec<T> = Vec::new();
            for i in 0..self.rows {
                let mut accumulator: T = T::zero();

                for old_columns in 0..self.columns {
                    for old_rows in 0..rhs.len {
                        accumulator = accumulator.add(&(self.values[old_columns][i].mul(&rhs.values[old_rows])));
                    }
                }

                temp_vector.push(accumulator);
            }

            Vector::new(temp_vector)
            
        } else {
            panic!("Cannot multiplicate those matrices");
        }
    }
}


impl<T> std::ops::Div for Matrix<T> where T: Instance + Clone + Operand + PartialEq + Number {
    type Output = Either<Matrix<T>, Vector<T>>;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.rows == rhs.columns {
            self * (rhs.inverse())
        } else {
            panic!("Cannot compute the inverse of a non square matrix");
        }
    }
}



/*
*   Matrix functions for Univariate Polynomials
*
*/
impl<T> Matrix<UnivariatePolynomialInstance<T>> where T: Instance + Clone + PartialEq + Operand + Eq + Number + ClassInstance + 'static{


    pub fn get_dimension(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn inplace_transpose(&mut self) {
        if self.rows == self.columns {
            let values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    self.values[c][r] = values_copy[r][c].clone();
                }
            }
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            self.values = values
        }
    }

    pub fn transpose(&self) -> Matrix<UnivariatePolynomialInstance<T>> {
        if self.rows == self.columns {
            let mut values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    values_copy[c][r] = self.values[r][c].clone();
                }
            }

            Matrix::new(values_copy, self.rows, self.columns)
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            Matrix::new(values, self.columns, self.rows)
        }
    }

    pub fn prepend_row(&mut self, row: Vec<UnivariatePolynomialInstance<T>>) {
        if row.len() != self.columns {
            panic!("Cannot prepend row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].insert(0, row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn append_row(&mut self,row: Vec<UnivariatePolynomialInstance<T>>) {
        if row.len() != self.columns {
            panic!("Cannot appned row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].push(row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn prepend_column(&mut self, column: Vec<UnivariatePolynomialInstance<T>>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.insert(0, column);
        self.columns = self.columns + 1;
    }

    pub fn append_column(&mut self, column: Vec<UnivariatePolynomialInstance<T>>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.push(column);
        self.columns = self.columns + 1;
    }


    pub fn get_ith_vector(&self, index: usize) -> Vector<UnivariatePolynomialInstance<T>> {
        if index < self.columns {
            let vector: Vec<UnivariatePolynomialInstance<T>> = self.values[index].clone();
            Vector::new(vector)
        } else {
            panic!("Index out of range");
        }
    }

    // i: row, j: column
    pub fn get_minor_matrix(&self, i: usize, j: usize) -> Matrix<UnivariatePolynomialInstance<T>> {
        let mut container: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
        for c in j..self.columns {
            container.push(self.values[c][i..].to_vec());
        }

        let rows = self.rows - i;
        let columns = self.columns - j;

        Matrix::new(container, rows, columns)
    }


    pub fn determinant(&self) -> UnivariatePolynomialInstance<T> {
        if self.rows != self.columns {
            panic!("Determinant cannot be computed for non square matrices");
        }

        if self.rows == 2 && self.columns == 2 {
            return (self.values[0][0].mul(&self.values[1][1])).sub(&(self.values[0][1].mul(&self.values[1][0])));
        } else {
            let variable: Var = self.values[0][0].var.clone();
            let generator: Box<dyn StatefulClass> = self.values[0][0].coefficients[0].get_class();
            let mut determinant: UnivariatePolynomialInstance<T> = UnivariatePolynomial::zero(variable, &generator);

            for r in 0..self.rows {
                if r & 0x1 == 1 {
                    determinant = determinant.add(&((self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))).neg()));
                } else {
                    determinant = determinant.add(&(self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))));
                }
            }

            return determinant;
        }
    }

    pub fn inverse(&self) -> Matrix<UnivariatePolynomialInstance<T>>{
        if self.rows != self.columns {
            panic!("Cannot compute inverse of non square matrices");
        }
        let variable: Var = self.values[0][0].var.clone();

        let determinant = self.determinant();
        let generator: Box<dyn StatefulClass> = self.values[0][0].coefficients[0].get_class();

        if determinant == UnivariatePolynomial::zero(variable, &generator) {
            panic!("[ERROR] Determinant of matrix is zero");
        }

        // special case for 2x2 matrices
        if self.rows == 2 && self.columns == 2 {
            let mut container: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
            let mut vector1: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
            vector1.push(self.values[1][1].div(&determinant));
            vector1.push((self.values[1][0].div(&determinant)).neg());
            let mut vector2: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
            vector2.push((self.values[0][1].div(&determinant)).neg());
            vector2.push(self.values[0][0].div(&determinant));

            container.push(vector1);
            container.push(vector2);
            return Matrix::new(container, 2, 2)
        }

        // find matrix of cofactors
        let mut cofactors: Matrix<UnivariatePolynomialInstance<T>> = Matrix::new(self.values.clone(), self.rows, self.columns);
        for r in 0..self.rows {
            let mut cofactor_row: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
            for c in 0..self.columns {
                let minor_matrix: Matrix<UnivariatePolynomialInstance<T>> = self.get_minor_matrix(r, c);
                let minor_determinant: UnivariatePolynomialInstance<T> = minor_matrix.determinant();
                let exponent = (r+c) & 0x1;
                if exponent == 1 {
                    cofactor_row.push(minor_determinant.neg());
                } else {
                    cofactor_row.push(minor_determinant);
                }
            }

            cofactors.append_row(cofactor_row);
        }

        cofactors.transpose();

        for c in 0..cofactors.columns {
            for r in 0..cofactors.rows {
                cofactors.values[r][c] = cofactors.values[r][c].div(&determinant);
            }
        }

        cofactors
    }


    // pub fn zero_matrix(rows: usize, columns: usize) -> Matrix<UnivariatePolynomialInstance<T>> {
    //     let mut container: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
    //     let variable: Var = Var::new("x", BigInt::from(1));
    //     let generator: Box<dyn StatefulClass> = T::get_class(&self);
    //     for _c in 0..columns {
    //         container.push(vec![UnivariatePolynomial::zero(variable.clone()); rows]);
    //     }

    //     Matrix::new(container, rows, columns)
    // }
    // pub fn identity_matrix(dim: usize) -> Matrix<UnivariatePolynomialInstance<T>> {
    //     let mut container: Vec<Vec<UnivariatePolynomialInstance<T>>> = Vec::new();
    //     let variable: Var = Var::new("x", BigInt::from(1));
    //     for c in 0..dim {
    //         let mut vector: Vec<UnivariatePolynomialInstance<T>> = Vec::new();
    //         for r in 0..dim {
    //             if r == c {
    //                 vector.push(UnivariatePolynomial::one(variable.clone()));
    //             } else {
    //                 vector.push(UnivariatePolynomial::zero(variable.clone()));
    //             }
    //         }
    //         container.push(vector);
    //     }

    //     Matrix::new(container, dim, dim)
    // }
    

}


/***
 *  Matrix functions for Polynomial Rings
 * 
 */
impl<T> Matrix<PolynomialRingInstance<T>> where T: Instance + Clone + PartialEq + Operand + Eq + Number + ClassInstance + 'static{


    pub fn get_dimension(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn inplace_transpose(&mut self) {
        if self.rows == self.columns {
            let values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    self.values[c][r] = values_copy[r][c].clone();
                }
            }
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<PolynomialRingInstance<T>> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            self.values = values
        }
    }

    pub fn transpose(&self) -> Matrix<PolynomialRingInstance<T>> {
        if self.rows == self.columns {
            let mut values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    values_copy[c][r] = self.values[r][c].clone();
                }
            }

            Matrix::new(values_copy, self.rows, self.columns)
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<PolynomialRingInstance<T>> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            Matrix::new(values, self.columns, self.rows)
        }
    }

    pub fn prepend_row(&mut self, row: Vec<PolynomialRingInstance<T>>) {
        if row.len() != self.columns {
            panic!("Cannot prepend row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].insert(0, row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn append_row(&mut self,row: Vec<PolynomialRingInstance<T>>) {
        if row.len() != self.columns {
            panic!("Cannot appned row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].push(row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn prepend_column(&mut self, column: Vec<PolynomialRingInstance<T>>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.insert(0, column);
        self.columns = self.columns + 1;
    }

    pub fn append_column(&mut self, column: Vec<PolynomialRingInstance<T>>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.push(column);
        self.columns = self.columns + 1;
    }


    pub fn get_ith_vector(&self, index: usize) -> Vector<PolynomialRingInstance<T>> {
        if index < self.columns {
            let vector: Vec<PolynomialRingInstance<T>> = self.values[index].clone();
            Vector::new(vector)
        } else {
            panic!("Index out of range");
        }
    }

    // i: row, j: column
    pub fn get_minor_matrix(&self, i: usize, j: usize) -> Matrix<PolynomialRingInstance<T>> {
        let mut container: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();
        for c in j..self.columns {
            container.push(self.values[c][i..].to_vec());
        }

        let rows = self.rows - i;
        let columns = self.columns - j;

        Matrix::new(container, rows, columns)
    }


    pub fn determinant(&self) -> PolynomialRingInstance<T> {
        if self.rows != self.columns {
            panic!("Determinant cannot be computed for non square matrices");
        }

        if self.rows == 2 && self.columns == 2 {
            return (self.values[0][0].mul(&self.values[1][1])).sub(&(self.values[0][1].mul(&self.values[1][0])));
        } else {
            let variable: Var = self.values[0][0].var.clone();
            let generator: PolynomialRing<T> = self.values[0][0].clone().class.into_inner().clone();
            let generator_values: Box<dyn StatefulClass> = self.values[0][0].clone().coefficients[0].get_class();

            let mut determinant: PolynomialRingInstance<T> = generator.zero(variable, &generator_values);

            for r in 0..self.rows {
                if r & 0x1 == 1 {
                    determinant = determinant.add(&((self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))).neg()));
                } else {
                    determinant = determinant.add(&(self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))));
                }
            }

            return determinant;
        }
    }

    pub fn inverse(&self) -> Matrix<PolynomialRingInstance<T>>{
        if self.rows != self.columns {
            panic!("Cannot compute inverse of non square matrices");
        }
        let variable: Var = self.values[0][0].var.clone();
        let generator: PolynomialRing<T> = self.values[0][0].clone().class.into_inner().clone();
        let generator_values: Box<dyn StatefulClass> = self.values[0][0].clone().coefficients[0].get_class();

        let determinant = self.determinant();
        if determinant == generator.zero(variable, &generator_values) {
            panic!("[ERROR] Determinant of matrix is zero");
        }

        // special case for 2x2 matrices
        if self.rows == 2 && self.columns == 2 {
            let mut container: Vec<Vec<PolynomialRingInstance<T>>> = Vec::new();
            let mut vector1: Vec<PolynomialRingInstance<T>> = Vec::new();
            vector1.push(self.values[1][1].div(&determinant));
            vector1.push((self.values[1][0].div(&determinant)).neg());
            let mut vector2: Vec<PolynomialRingInstance<T>> = Vec::new();
            vector2.push((self.values[0][1].div(&determinant)).neg());
            vector2.push(self.values[0][0].div(&determinant));

            container.push(vector1);
            container.push(vector2);
            return Matrix::new(container, 2, 2)
        }

        // find matrix of cofactors
        let mut cofactors: Matrix<PolynomialRingInstance<T>> = Matrix::new(self.values.clone(), self.rows, self.columns);
        for r in 0..self.rows {
            let mut cofactor_row: Vec<PolynomialRingInstance<T>> = Vec::new();
            for c in 0..self.columns {
                let minor_matrix: Matrix<PolynomialRingInstance<T>> = self.get_minor_matrix(r, c);
                let minor_determinant: PolynomialRingInstance<T> = minor_matrix.determinant();
                let exponent = (r+c) & 0x1;
                if exponent == 1 {
                    cofactor_row.push(minor_determinant.neg());
                } else {
                    cofactor_row.push(minor_determinant);
                }
            }

            cofactors.append_row(cofactor_row);
        }

        cofactors.transpose();

        for c in 0..cofactors.columns {
            for r in 0..cofactors.rows {
                cofactors.values[r][c] = cofactors.values[r][c].div(&determinant);
            }
        }

        cofactors
    }


    
    
    

}

/**
 *  Matrix multiplication for Numerics
 */
impl<T> Matrix<T> where T: Instance + Clone + PartialEq + Operand + Number {


    pub fn get_dimension(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }

    pub fn inplace_transpose(&mut self) {
        if self.rows == self.columns {
            let values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    self.values[c][r] = values_copy[r][c].clone();
                }
            }
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<T>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<T> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            self.values = values
        }
    }

    pub fn transpose(&self) -> Matrix<T> {
        if self.rows == self.columns {
            let mut values_copy = self.values.clone();
            for r in 0..self.rows {
                for c in 0..self.columns {
                    values_copy[c][r] = self.values[r][c].clone();
                }
            }

            Matrix::new(values_copy, self.rows, self.columns)
        } else {
            // in this way all the columns will be modified
            let mut values: Vec<Vec<T>> = Vec::new();
            for r in 0..self.rows {
                let mut temp_vector: Vec<T> = Vec::new();
                for c in 0..self.columns {
                    temp_vector.push(self.values[c][r].clone());
                }
                values.push(temp_vector)
            }

            Matrix::new(values, self.columns, self.rows)
        }
    }

    pub fn prepend_row(&mut self, row: Vec<T>) {
        if row.len() != self.columns {
            panic!("Cannot prepend row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].insert(0, row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn append_row(&mut self,row: Vec<T>) {
        if row.len() != self.columns {
            panic!("Cannot appned row with different number of columns");
        }

        for i in 0..row.len() {
            self.values[i].push(row[i].clone());
        }

        self.rows = self.rows + 1;
    }

    pub fn prepend_column(&mut self, column: Vec<T>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.insert(0, column);
        self.columns = self.columns + 1;
    }

    pub fn append_column(&mut self, column: Vec<T>) {
        if column.len() != self.rows {
            panic!("Cannot prepend column with different number of rows");
        }

        self.values.push(column);
        self.columns = self.columns + 1;
    }


    pub fn get_ith_vector(&self, index: usize) -> Vector<T> {
        if index < self.columns {
            let vector: Vec<T> = self.values[index].clone();
            Vector::new(vector)
        } else {
            panic!("Index out of range");
        }
    }

    // i: row, j: column
    pub fn get_minor_matrix(&self, i: usize, j: usize) -> Matrix<T> {
        let mut container: Vec<Vec<T>> = Vec::new();
        for c in j..self.columns {
            container.push(self.values[c][i..].to_vec());
        }

        let rows = self.rows - i;
        let columns = self.columns - j;

        Matrix::new(container, rows, columns)
    }


    pub fn determinant(&self) -> T {
        if self.rows != self.columns {
            panic!("Determinant cannot be computed for non square matrices");
        }

        if self.rows == 2 && self.columns == 2 {
            return (self.values[0][0].mul(&self.values[1][1])).sub(&(self.values[0][1].mul(&self.values[1][0])));
        } else {

            let mut determinant: T = T::zero();

            for r in 0..self.rows {
                if r & 0x1 == 1 {
                    determinant = determinant.add(&((self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))).neg()));
                } else {
                    determinant = determinant.add(&(self.values[0][r].mul(&(self.get_minor_matrix(0, r).determinant()))));
                }
            }

            return determinant;
        }
    }

    pub fn inverse(&self) -> Matrix<T>{
        if self.rows != self.columns {
            panic!("Cannot compute inverse of non square matrices");
        }

        let determinant = self.determinant();
        if determinant == T::zero() {
            panic!("[ERROR] Determinant of matrix is zero");
        }

        // special case for 2x2 matrices
        if self.rows == 2 && self.columns == 2 {
            let mut container: Vec<Vec<T>> = Vec::new();
            let mut vector1: Vec<T> = Vec::new();
            vector1.push(self.values[1][1].div(&determinant));
            vector1.push((self.values[1][0].div(&determinant)).neg());
            let mut vector2: Vec<T> = Vec::new();
            vector2.push((self.values[0][1].div(&determinant)).neg());
            vector2.push(self.values[0][0].div(&determinant));

            container.push(vector1);
            container.push(vector2);
            return Matrix::new(container, 2, 2)
        }

        // find matrix of cofactors
        let mut cofactors: Matrix<T> = Matrix::new(self.values.clone(), self.rows, self.columns);
        for r in 0..self.rows {
            let mut cofactor_row: Vec<T> = Vec::new();
            for c in 0..self.columns {
                let minor_matrix: Matrix<T> = self.get_minor_matrix(r, c);
                let minor_determinant: T = minor_matrix.determinant();
                let exponent = (r+c) & 0x1;
                if exponent == 1 {
                    cofactor_row.push(minor_determinant.neg());
                } else {
                    cofactor_row.push(minor_determinant);
                }
            }

            cofactors.append_row(cofactor_row);
        }

        cofactors.transpose();

        for c in 0..cofactors.columns {
            for r in 0..cofactors.rows {
                cofactors.values[r][c] = cofactors.values[r][c].div(&determinant);
            }
        }

        cofactors
    }


    pub fn zero_matrix(rows: usize, columns: usize) -> Matrix<T> {
        let mut container: Vec<Vec<T>> = Vec::new();
        for _c in 0..columns {
            container.push(vec![T::zero(); rows]);
        }

        Matrix::new(container, rows, columns)
    }
    pub fn identity_matrix(dim: usize) -> Matrix<T> {
        let mut container: Vec<Vec<T>> = Vec::new();
        for c in 0..dim {
            let mut vector: Vec<T> = Vec::new();
            for r in 0..dim {
                if r == c {
                    vector.push(T::one());
                } else {
                    vector.push(T::zero());
                }
            }
            container.push(vector);
        }

        Matrix::new(container, dim, dim)
    }
    

}

