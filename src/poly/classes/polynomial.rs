use std::cell::RefCell;

use num_bigint::BigInt;

use crate::numbers::numbers::Class;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::instances::monomial_instance;
use crate::poly::instances::monomial_instance::MonomialInstance;
use crate::poly::instances::polynomial_instance::PolynomialInstance;
use crate::variables::vars::Var;



// POLYNOMIAL
#[derive(Clone)]
pub struct Polynomial {
    class: ClassTypes
}

impl Polynomial {
    pub fn new<T>(monomials: Vec<MonomialInstance<T>>) -> PolynomialInstance<T> where T: Instance + Operand + Clone {
        PolynomialInstance::<T> { class: RefCell::new(Polynomial {class: ClassTypes::Polynomial}), monomials: monomials }
    }

    // pub fn ToUnivariatePolynomial(self) -> UnivariatePolynomial<T> {
    //     // write the code to convert a polynomial to univariate polynomial
    // }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
        
    }
}
impl Eq for Polynomial {}


// add function

// mul function

// div function

// sub function

// pow to BigInt


