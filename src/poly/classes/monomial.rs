use std::cell::RefCell;

use crate::numbers::numbers::Number;
use crate::poly::instances::monomial_instance::MonomialInstance;
use crate::variables::vars::Var;
use num_bigint::BigInt;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use either::*;
use num_traits::Pow;
use crate::poly::instances::polynomial_instance::PolynomialInstance;
use crate::numbers::sets::Class::ClassTypes;

use super::polynomial::Polynomial;
// MONOMIALS
// create struct MONOMI which will compose the polynomial
#[derive(Clone)]
pub struct Monomial {
    pub class: ClassTypes
}


impl Monomial {
    pub fn new() -> Self {
        Monomial {class: ClassTypes::Monomial}
    }
    pub fn new_monomial<T>(vars: Vec<Var>, coefficient: T) -> MonomialInstance<T> where T: Instance + Operand + Clone + Number {
        MonomialInstance { class: RefCell::new(Monomial::new()), variables: vars, coefficient: coefficient }
    }

    pub fn new_from_var<T>(a: Var) -> MonomialInstance<T> where T: Instance + Operand + Clone + Number{
        // implement function
        let mut vars: Vec<Var> = Vec::new();
        vars.push(a);
        MonomialInstance { class: RefCell::new(Monomial::new()), variables: vars, coefficient: T::one()}
    }

}

impl PartialEq for Monomial {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class   
    }
}
impl Eq for Monomial {}

// operations
impl Monomial {


    pub fn neg<T>(x: MonomialInstance<T>) -> MonomialInstance<T> where T: Instance + Operand + Clone + Number {
        Monomial::new_monomial(x.variables, x.coefficient.neg())
        
    }
    pub fn add<T>(x: MonomialInstance<T>, y: MonomialInstance<T>) -> Either<MonomialInstance<T>, PolynomialInstance<T>> where T: Instance + Operand + Clone + Number {
        if x.is_similar(y.clone()) {
            let k: T = y.coefficient;
            Left(Monomial::new_monomial(x.variables, x.coefficient.add(&k)))
        } else {
            //create polynomial
            let mut monomials = Vec::new();
            monomials.push(x);
            monomials.push(y);
            Right(Polynomial::new(monomials))
        }
    }

    pub fn sub<T>(x: MonomialInstance<T>, y: MonomialInstance<T>) -> Either<MonomialInstance<T>, PolynomialInstance<T>> where T: Instance + Operand + Clone + Number{
        if x.is_similar(y.clone()) {
            let k: T = y.coefficient;
            Left(Monomial::new_monomial(x.variables, x.coefficient.sub(&k)))
        } else {
            //create polynomial
            let mut monomials = Vec::new();
            monomials.push(x);
            monomials.push(-y);
            Right(Polynomial::new(monomials))
        }
    }

    pub fn mul<T>(x: MonomialInstance<T>, y: MonomialInstance<T>) -> MonomialInstance<T> where T: Instance + Operand + Clone + Number{
        let k: T = y.coefficient;
        let mut variables_clone = x.variables.clone();
        for i in y.variables {
            let mut new: bool = true;
            for j in &mut variables_clone {
                if i == *j {
                    new = false;
                    *j = (*j).clone()*i.clone();
                    break;
                } else {}
            }
            if new == true {
                variables_clone.push(i)
            } else {}
        }
        Monomial::new_monomial(variables_clone, x.coefficient.mul(&k))
    }

    pub fn div<T>(x: MonomialInstance<T>, y: MonomialInstance<T>) -> MonomialInstance<T> where T: Instance + Operand + Clone + Number{
        let k: T = y.coefficient;
        let mut variables_clone = x.variables.clone();
        for i in y.variables {
            let mut new: bool = true;
            for j in &mut variables_clone {
                if i == *j {
                    new = false;
                    *j = (*j).clone()/i.clone();
                    break;
                } else {}
            }
            if new == true {
                variables_clone.push(i.pow(BigInt::from(-1)))
            } else {}
        }
        Monomial::new_monomial(variables_clone, x.coefficient.div(&k))
    }
}



