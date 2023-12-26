//use sagemath::numbers::sets::General_Class;

use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::utilities::utils;
use std::cell::RefCell;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::poly::univariate_polynomial::UnivariatePolynomial;
use crate::variables::vars::Var;
// wrapper on ZZ_instance
#[derive(Clone)]
pub struct PolynomialRing<T> {
    pub irreducible_polynomial: UnivariatePolynomial<T>
}


// impl<T> class<PolynomialRing_instance<T>> for PolynomialRing<T> where T: Instance {
//     // need to implement apply and has_type
//     fn apply(&self, value: T) -> PolynomialRing_instance<T> {
//         match value.has_type() {
//             ClassTypes::UnivariatePolynomial => self.new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone()),
//             _ => self.new_instance(BigInt::from(0))
//         }
//     }


//     fn applyToMonomial(&self, monomial: Monomial<T>) -> Monomial<Zmod_instance> {
//         Monomial::new(monomial.variables, self.apply(monomial.coefficient))
//     }


//     fn has_type(&self) -> ClassTypes {
//         ClassTypes::Zmod
//     }
// }

impl<T> PolynomialRing<T> where T: Instance + Clone + PartialEq + Operand + Number {
    pub fn apply(&self, x: &UnivariatePolynomial<T>) -> PolynomialRingInstance<T> {
        let qr: Vec<UnivariatePolynomial<T>> = utils::poly_divmod(x, &(self.irreducible_polynomial));
        self.new_instance(qr[1].var.clone(), qr[1].coefficients.clone())
    }
}


impl<T> PartialEq for PolynomialRing<T> where T: Instance + PartialEq + Clone{
    fn eq(&self, other: &Self) -> bool {
        self.irreducible_polynomial == other.irreducible_polynomial
    }
}
impl<T> Eq for PolynomialRing<T> where T: Instance + PartialEq + Clone {}

impl<T> PolynomialRing<T> where T: Instance + Operand + Clone + PartialEq + Number {
    pub fn one(self, v: Var) -> PolynomialRingInstance<T> {
        self.new_instance(v, vec![T::one()])
    }

    pub fn zero(self, v: Var) -> PolynomialRingInstance<T> {
        self.new_instance(v, vec![T::zero()])
    }
}


impl<T> PolynomialRing<T> where T: Instance + Operand + Clone + PartialEq + Number {
    pub fn new(irreducible_polynomial: UnivariatePolynomial<T>) -> PolynomialRing<T> {
        PolynomialRing { irreducible_polynomial: irreducible_polynomial } 
    }

    pub fn new_instance(&self, var: Var, coefficients: Vec<T>) -> PolynomialRingInstance<T> {
        PolynomialRingInstance { class: RefCell::new(self.clone()), var: var, coefficients: coefficients } 
    }

    pub fn add(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
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
                coeff.push(x.coefficients[i].clone());
            }
        } else {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
            }
        }

        self.new_instance(x.var, coeff) 


    }

    pub fn sub(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
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
                coeff.push(x.coefficients[i].clone());
            }
        } else {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
            }
        }

        self.new_instance(x.var, coeff) 
    }

    pub fn mul(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>)-> PolynomialRingInstance<T>  {
        // schoolbook multiplication, then reducing by irreducible poly thanks to divmod
        let len = x.coefficients.len() + y.coefficients.len() -1;
        let mut coeff: Vec<T> = vec![T::zero(); len];
        
        // schoolbook multiplication
        for i in 0..x.coefficients.len() {
            // perform self[i] * rhs
            for j in 0..y.coefficients.len() {
                coeff[i+j] = coeff[i+j].clone().add(&(x.coefficients[i].clone().mul(&(y.coefficients[j]))));
            }
        }

        let current_poly = UnivariatePolynomial::new(coeff, x.var.clone(), None);
        self.apply(&current_poly)
    }

    pub fn div(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T>  {
        x * y.inverse()
    }

    pub fn neg(&self, x: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let coefficients = x.coefficients.into_iter().map(| x| {
            x.neg()
        }).collect();

        self.new_instance(x.var, coefficients)   
    }

    pub fn inverse(&self, x: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let result: Vec<PolynomialRingInstance<T>> = utils::egcd(x.clone(), self.apply(&self.irreducible_polynomial));
        if result[0] != self.clone().one(x.var) {
            panic!("The inverse does not exist");
        } else {
            return result[1].clone()
        }
    }
}


