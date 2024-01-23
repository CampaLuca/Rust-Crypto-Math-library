use std::cell::RefCell;

use num_bigint::BigInt;

use crate::numbers::numbers::Class;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::classes::polynomial::Polynomial;
use crate::poly::instances::monomial_instance::MonomialInstance;
use crate::variables::vars::Var;



// POLYNOMIAL
#[derive(Clone)]
pub struct PolynomialInstance<T> {
    pub class: RefCell<Polynomial>,
    pub monomials: Vec<MonomialInstance<T>>
}

impl<T> PolynomialInstance<T> where T: Instance + Operand + Clone {
   

    // pub fn ToUnivariatePolynomial(self) -> UnivariatePolynomial<T> {
    //     // write the code to convert a polynomial to univariate polynomial
    // }
}

impl<T> PartialEq for PolynomialInstance<T> where T: Instance + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.monomials.len() != other.monomials.len() {
            return false;
        }

        let mut equal: bool = true;
        for i in 0..self.monomials.len() {
            equal = equal && (self.monomials[i] == other.monomials[i]);
        }

        equal
        
    }
}
impl<T> Eq for PolynomialInstance<T> where T: Instance + PartialEq {}


// add function

// mul function

// div function

// sub function

// pow to BigInt




impl<T> std::fmt::Display for PolynomialInstance<T> where T: Instance + Operand + Clone +  std::fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("");

        for i in &self.monomials {
            let monomial = format!("{i}");
            printable = format!("{printable}{monomial}")
        }

        write!(f, "{0}", printable )
    }
}


impl<T> Operand for PolynomialInstance<T> where T: Instance + Operand + Clone + Eq + Number {
    fn neg(&self) -> Self {
        let mut monomials: Vec<MonomialInstance<T>> = self.monomials.clone();
        for i in 0..monomials.len() {
            monomials[i] = -monomials[i].clone();
        }

        Polynomial::new(monomials)
    }
    
    fn add(&self, other: &Self) -> Self {
        self.clone() //*self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        self.clone() //*self - *other
    }

    fn mul(&self, other: &Self) -> Self {
        self.clone() //*self * *other
    }

    fn div(&self, other: &Self) -> Self {
        self.clone() //*self / *other
    }

    fn equal(&self, other:&Self) -> bool {
        *self == *other
    }

    // not implemented
    fn greater_than(&self, other: &Self) -> bool {
        panic!("This method is not implemented for Polynomials");
    }

    fn less_than(&self, other: &Self) -> bool {
        panic!("This method is not implemented for Polynomials");
    }
}


impl<T> Instance for PolynomialInstance<T> where T: Instance + Clone + Operand + Eq {
    fn as_any(&self) -> &dyn std::any::Any {
        panic!("Method not implemented"); //self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::Polynomial
    }

  
}


// impl<T> ClassInstance for PolynomialInstance<T> where T: Instance{
//     fn get_class(&self) -> Polynomial {
//         self.class
//     }
// }

    // fn is_zero(self) -> bool {
    //     self.monomials.len() == 0
    // }

    // fn one() -> Self {
    //     let mut monomials: Vec<Monomial<T>> = Vec::new();
    //     let mut variables: Vec<Var> = Vec::new();
    //     variables.push(Var::new("x", BigInt::from(0)));
    //     monomials.push(Monomial::new(variables, T::one()));
    //     Polynomial::new(monomials)
    // }

    // fn zero() -> Self {
    //     let monomials: Vec<Monomial<T>> = Vec::new();
    //     Polynomial::new(monomials)
    // }

    // fn random(_bit_length: u64) -> Self {
    //     panic!("If you want to generate random polynomials don't use this method");
    // }

    // fn random_with_bounds(_lower_bound: BigInt, _upper_bound: BigInt) -> Self {
    //     panic!("If you want to generate random polynomials don't use this method");
    // }
