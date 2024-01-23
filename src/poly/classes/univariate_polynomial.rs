use std::cell::RefCell;

use num_bigint::BigInt;
use num_traits::Num;

use crate::algebras::FiniteField::classes::Zmod::Zmod;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::arith::random::get_random_bigint_with_bounds;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::instances::ZZ_instance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::numbers::Number;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use crate::utilities;
use crate::utilities::utils::poly_divmod;
use crate::variables::vars::Var;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;

// utilities
fn clean<T>(mut coeff: Vec<T>) -> Vec<T> where T: Instance + Operand + Clone + Number {
    loop {
        if coeff[coeff.len()-1].clone().is_zero() && coeff.len() > 1 {
            coeff.pop();
        } else {
            break;
        }
    }
    coeff
}

pub enum PolyMultiplicationAlgorithm {
    ToomCook,
    Naive,
    EvaluationForm
}

// POLYNOMIAL
#[derive(Clone)]
pub struct UnivariatePolynomial {
    pub class: ClassTypes,
    pub multiplication_algorithm: Option<String>
}

impl PartialEq for UnivariatePolynomial {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.multiplication_algorithm == other.multiplication_algorithm
    }
}
impl Eq for UnivariatePolynomial {}

impl UnivariatePolynomial {
    pub fn new(multiplication_algorithm: Option<String>) -> UnivariatePolynomial {
        UnivariatePolynomial {class: ClassTypes::UnivariatePolynomial, multiplication_algorithm}
    }


    pub fn new_instance<T>(coefficients: Vec<T>, var: Var, multiplication_algorithm: Option<String>) -> UnivariatePolynomialInstance<T> where T: Instance + Clone + PartialEq + Operand + Number {
        let class: UnivariatePolynomial = UnivariatePolynomial::new(multiplication_algorithm.clone());
        
        if multiplication_algorithm.is_none() {
            UnivariatePolynomialInstance { class: RefCell::new(class), coefficients: clean::<T>(coefficients), var: var }
        } else {
            UnivariatePolynomialInstance { class: RefCell::new(class), coefficients: clean::<T>(coefficients), var: var }
        }
    }

    pub fn one<T>(var: Var) -> UnivariatePolynomialInstance<T> where T: Number + Instance + Clone + PartialEq + Operand{
        // let variable = Var::new("x", BigInt::from(0));
        let mut coefficients: Vec<T> = Vec::new();
        coefficients.push(T::one());
        UnivariatePolynomial::new_instance(coefficients, var, None)
    }

    pub fn zero<T>(var: Var) -> UnivariatePolynomialInstance<T> where T: Number + Instance + Clone + PartialEq + Operand {
        // let variable = Var::new("x", BigInt::from(0));
        let mut coefficients: Vec<T> = Vec::new();
        coefficients.push(T::zero());
        UnivariatePolynomial::new_instance(coefficients, var, None)
    }
    
}

impl UnivariatePolynomial {
    pub fn neg<T>(x: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number{
        let mut coefficients: Vec<T> = x.coefficients.clone();
        for i in 0..coefficients.len() {
            coefficients[i] = coefficients[i].neg();
        }

        UnivariatePolynomial::new_instance(coefficients, x.var.clone(), x.class.into_inner().multiplication_algorithm)
    }

    pub fn add<T>(x: UnivariatePolynomialInstance<T>, y: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number{
        if x.var == y.var {

            let mut coeff = Vec::new();
            if x.coefficients.len() > y.coefficients.len() {
                for i in 0..y.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
                }
                for i in y.coefficients.len()..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone());
                }
            } else if x.coefficients.len() < y.coefficients.len() {
                for i in 0..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
                }
                for i in x.coefficients.len()..y.coefficients.len() {
                    coeff.push(y.coefficients[i].clone());
                }
            } else {
                for i in 0..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
                }
            }

            UnivariatePolynomial::new_instance(coeff, x.var.clone(), x.class.into_inner().multiplication_algorithm)
        } else {
            panic!("Cannot add these polynomials")
        }
    }

    pub fn sub<T>(x: UnivariatePolynomialInstance<T>, y: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number{
        if  x.var == y.var {
            let mut coeff = Vec::new();
            if x.coefficients.len() > y.coefficients.len() {
                for i in 0..y.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
                }
                for i in y.coefficients.len()..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone());
                }
            } else if x.coefficients.len() < y.coefficients.len() {
                for i in 0..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
                }
                for i in x.coefficients.len()..y.coefficients.len() {
                    coeff.push(y.coefficients[i].clone().neg());
                }
            } else {
                for i in 0..x.coefficients.len() {
                    coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
                }
            }

            UnivariatePolynomial::new_instance(coeff, x.var.clone(), x.class.into_inner().multiplication_algorithm)
        } else {
            panic!("ERROR: Cannot sub these polynomials")
        }
    }


    pub fn mul<T>(x: UnivariatePolynomialInstance<T>, y: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number{
        if x.var == y.var {
            // SCHOOLBOOK MULTIPLICATION
            //if x.multiplication_algorithm == "Naive" {
                let len = x.coefficients.len() + y.coefficients.len() -1;
                let mut coeff: Vec<T> = vec![T::zero(); len];
                
                // schoolbook multiplication
                for i in 0..x.coefficients.len() {
                    // perform x[i] * y
                    for j in 0..y.coefficients.len() {
                        coeff[i+j] = coeff[i+j].clone().add(&(x.coefficients[i].clone().mul(&(y.coefficients[j]))));
                    }
                }
                
                return UnivariatePolynomial::new_instance(coeff, x.var.clone(), x.class.into_inner().multiplication_algorithm)
            // } else if x.multiplication_algorithm == "EvaluationMethod" {
            //     // need a random function
            //     // get degree of polynomials
            //     // get higher degree points
            //     // evaluate points
            //     // lagrange interpolation
            // }


        } else {
            panic!("Cannot multiply those 2 polynomials");
        }


    }

    pub fn mul_by_scalar<T>(x: UnivariatePolynomialInstance<T>, y: T) -> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
        let coefficients = x.coefficients.into_iter().map(| x| {
            x//x.mul(&y)
        }).collect();
        UnivariatePolynomial::new_instance(coefficients, x.var, x.class.into_inner().multiplication_algorithm)
    }

    pub fn rem<T>(x: UnivariatePolynomialInstance<T>, y: ZZinstance) -> UnivariatePolynomialInstance<ZmodInstance> where T: Instance + Operand + Clone + PartialEq + Number  {
        let field: Zmod = Zmod::new(Some(y));
        let coefficients = x.coefficients.into_iter().map(| x| {
            field.apply(x)
        }).collect();
        UnivariatePolynomial::new_instance(coefficients, x.var, x.class.into_inner().multiplication_algorithm)
    }

    pub fn div<T>(x: UnivariatePolynomialInstance<T>, y: UnivariatePolynomialInstance<T>) -> (UnivariatePolynomialInstance<T>, UnivariatePolynomialInstance<T>) where T: Instance + Operand + Clone + PartialEq + Number + ClassInstance + 'static{
        let q_and_r: Vec<UnivariatePolynomialInstance<T>> = poly_divmod(&x.clone(), &y.clone());
        (q_and_r[0].clone(), q_and_r[1].clone())
    }

}



