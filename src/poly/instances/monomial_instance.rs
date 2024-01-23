use std::cell::RefCell;

use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Number;
use crate::poly::classes::monomial::Monomial;
use crate::variables::vars::Var;
use num_bigint::BigInt;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use either::*;
use num_traits::Pow;
use super::polynomial_instance::PolynomialInstance;

// MONOMIALS
// create struct MONOMI which will compose the polynomial
#[derive(Clone)]
pub struct MonomialInstance<T> {
    pub class: RefCell<Monomial>,
    pub variables: Vec<Var>,
    pub coefficient: T 
}


impl<T> MonomialInstance<T> where T: Instance + Operand + Clone + Number {
    
    pub fn is_similar(&self, other: MonomialInstance<T>) -> bool {
        if self.variables.len() == other.variables.len() && self.coefficient.has_type() == other.coefficient.has_type() {
            for i in 0..self.variables.len() {
                if self.variables[i] != other.variables[i] {
                    return false;
                }
            }

            return true;
        } 

        false
    }
}

impl<T> PartialEq for MonomialInstance<T> where T: Instance + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.variables.len() != other.variables.len() {
            return false;
        }

        let mut equal: bool = true;
        for i in 0..self.variables.len() {
            equal = equal && (self.variables[i] == other.variables[i]);
        }

        equal && (self.coefficient == other.coefficient)
        
    }
}
impl<T> Eq for MonomialInstance<T> where T: Instance + PartialEq {}


// OPERATIONS

impl<T> std::ops::Neg for MonomialInstance<T> where T: Instance + Operand + Clone + Number{
    type Output = MonomialInstance<T>;
    fn neg(self) -> MonomialInstance<T> {
        Monomial::neg(self)
    }
}

impl<T> std::ops::Add for MonomialInstance<T> where T: Instance + Operand + Clone + Number {
    type Output = Either<MonomialInstance<T>, PolynomialInstance<T>>;
    fn add(self, rhs: MonomialInstance<T>) -> Either<MonomialInstance<T>, PolynomialInstance<T>> {
        Monomial::add(self, rhs)
    }
}

impl<T> std::ops::Sub for MonomialInstance<T> where T: Instance + Operand + Clone + Number {
    type Output = Either<MonomialInstance<T>, PolynomialInstance<T>>;
    fn sub(self, rhs: MonomialInstance<T>) -> Either<MonomialInstance<T>, PolynomialInstance<T>> {
        Monomial::sub(self, rhs)

    }
}


// multiplication and division will generate a new monomial
impl<T> std::ops::Mul for MonomialInstance<T> where T: Instance + Operand + Clone + Number {
    type Output = MonomialInstance<T>;
    fn mul(self, rhs: MonomialInstance<T>) -> MonomialInstance<T> {
        Monomial::mul(self, rhs)
    }
}

impl<T> std::ops::Div for MonomialInstance<T> where T: Instance + Operand + Clone + Number {
    type Output = MonomialInstance<T>;
    fn div(self, rhs: MonomialInstance<T>) -> MonomialInstance<T> {
        Monomial::div(self, rhs)
    }
}


impl<T> std::fmt::Display for MonomialInstance<T> where T: Instance + Operand + Clone +  std::fmt::Display  {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("{0}", self.coefficient);
        

        for i in &self.variables {
            let variable = format!("{i}");
            printable = format!("{printable}{variable}")
        }

        write!(f, "{0}", printable )
    }
}

// impl<T> ClassInstance for MonomialInstance<T> where T: Instance{
//     fn get_class(&self) -> Monomial {
//         self.class
//     }
// }