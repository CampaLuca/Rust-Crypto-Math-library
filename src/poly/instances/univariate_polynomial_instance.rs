use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;

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
use crate::numbers::numbers::Number;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::utilities;
use crate::utilities::utils::poly_divmod;
use crate::variables::vars::Var;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;


pub enum PolyMultiplicationAlgorithm {
    ToomCook,
    Naive,
    EvaluationForm
}

// utilities
fn clean<T>(mut coeff: Vec<T>) -> Vec<T> where T: Instance + Operand + Clone + Number {
    loop {
        if coeff[coeff.len()-1].clone().is_zero() {
            coeff.pop();
        } else {
            break;
        }
    }
    coeff
}

// POLYNOMIAL
#[derive(Clone)]
pub struct UnivariatePolynomialInstance<T> {
    pub class: RefCell<UnivariatePolynomial>,
    pub coefficients: Vec<T>,
    pub var: Var,
    pub clean_coefficients: bool
}

impl<T> PartialEq for UnivariatePolynomialInstance<T> where T: Instance + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients && self.var == other.var && self.var == other.var
    }
}
impl<T> Eq for UnivariatePolynomialInstance<T> where T: Instance + PartialEq {}


impl<T> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number + ClassInstance + 'static + Display{
    pub fn quotient(self, irreducible_poly: UnivariatePolynomialInstance<T>, ntt_form: bool, fixed_length_coefficients: bool) -> PolynomialRingInstance<T> {
        let class: PolynomialRing<T> = PolynomialRing::new(irreducible_poly, fixed_length_coefficients);
        class.apply(&self,  ntt_form) //(self.var, self.coefficients)
    }
}

impl<T> UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
   

    
    pub fn round(&self) -> UnivariatePolynomialInstance<ZZinstance> {
        let mut coefficients: Vec<ZZinstance> = Vec::new();
        for i in 0..self.degree()+1 {
            coefficients.push((self.coefficients[i].clone()).round_to_zz());
        }
        UnivariatePolynomial::new_instance(coefficients, self.var.clone(), self.class.clone().into_inner().multiplication_algorithm, self.clean_coefficients)

    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()-1
    }

    pub fn leading_coefficient(&self) -> T {
        self.coefficients[self.coefficients.len()-1].clone()
    }
    
   

    pub fn one(v: Var) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::new_instance(vec![T::one()], v, None, true)
    }

    pub fn zero(v: Var) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::new_instance(vec![T::zero()], v, None, true)
    }
}


impl<T> std::ops::Neg for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomialInstance<T>;
    fn neg(self) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::neg(self)
    }
}

impl<T> std::ops::Add for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomialInstance<T>;
    fn add(self, rhs: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::add(self, rhs)
    }
}

impl<T> std::ops::Sub for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomialInstance<T>;
    fn sub(self, rhs: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::sub(self, rhs)
    }
}


impl<T> std::ops::Mul for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomialInstance<T>;
    fn mul(self, rhs: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::mul(self, rhs)
    }
}



// MULTIPLICATION by a scalar value
impl<T> std::ops::Mul<T> for UnivariatePolynomialInstance<T> where T: Instance + PartialEq  + Clone + Operand + Number {
    type Output = UnivariatePolynomialInstance<T>;
    fn mul(self, rhs: T) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::mul_by_scalar(self, rhs)
    }
}







impl std::ops::Rem<ZZinstance> for UnivariatePolynomialInstance<ZZinstance> {
    type Output = UnivariatePolynomialInstance<ZmodInstance>;
    fn rem(self, rhs: ZZinstance) -> Self::Output {
        UnivariatePolynomial::rem(self, rhs)
    }
}


impl<T> std::ops::Div for UnivariatePolynomialInstance<T> where T: 'static + Instance + Clone + PartialEq + Operand + Number + ClassInstance + Display {
    type Output = (UnivariatePolynomialInstance<T>, UnivariatePolynomialInstance<T>);
    fn div(self, rhs: UnivariatePolynomialInstance<T>) -> (UnivariatePolynomialInstance<T>, UnivariatePolynomialInstance<T>) {
        UnivariatePolynomial::div(self, rhs)
    }
}


impl<T> std::fmt::Display for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone +  Number + std::fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("");

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let mut sign: String = format!("");
            if coeff.less_than(&(T::zero())) {
                sign = format!("-");
            } 

            let mut power: String = format!("");
            if i > 0 {
                power = format!("x^{i}");
            }

            let mut coeff_string: String = format!("{coeff}");
            if coeff.equal(&(T::one())) {
                coeff_string = format!("");
            }
            printable = format!("{sign}{coeff_string}{power}{printable}")
        }

        write!(f, "{0}", printable)
    }
}



impl<T> Operand for UnivariatePolynomialInstance<T> where T: Instance + Operand + Clone + Eq + Number{
    fn neg(&self) -> Self {
        -self.clone()
    }
    
    fn add(&self, other: &Self) -> Self {
        self.clone() + other.clone()
    }

    fn sub(&self, other: &Self) -> Self {
        self.clone() - other.clone()
    }

    fn mul(&self, other: &Self) -> Self {
        self.clone() * other.clone()
    }

    fn div(&self, other: &Self) -> Self {
        panic!("Division implemented directly, not passing through operand");// *self / *other
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


impl<T> Instance for UnivariatePolynomialInstance<T> where T: Instance + Clone + Operand + Eq {
    fn as_any(&self) -> &dyn std::any::Any {
        panic!("Method not implemented");
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::UnivariatePolynomial
    }

}





// impl<T> Number for UnivariatePolynomialInstance<T> where T: Instance + Clone + Number + Operand + PartialEq{
//     fn is_zero(self) -> bool {
//         self.coefficients.len() == 1 && self.coefficients[0] == T::zero()
//     }

//     // fn one() -> Self {
//     //     let variable = Var::new("x", BigInt::from(0));
//     //     let mut coefficients: Vec<T> = Vec::new();
//     //     coefficients.push(T::one());
//     //     UnivariatePolynomial::new_instance(coefficients, variable, None)
//     // }

//     // fn zero() -> Self {
//     //     let variable = Var::new("x", BigInt::from(0));
//     //     let mut coefficients: Vec<T> = Vec::new();
//     //     coefficients.push(T::zero());
//     //     UnivariatePolynomial::new_instance(coefficients, variable, None)
//     // }

//     fn round_to_zz(self) -> ZZinstance {
//         panic!("Not implemented yet");
//     }
// }
    // fn is_zero(self) -> bool {
    //     self.coefficients.len() == 1 && self.coefficients[0] == T::zero()
    // }

    // fn one() -> Self {
    //     let variable = Var::new("x", BigInt::from(0));
    //     let mut coefficients: Vec<T> = Vec::new();
    //     coefficients.push(T::one());
    //     UnivariatePolynomial::new(coefficients, variable, None)
    // }

    // fn zero() -> Self {
    //     let variable = Var::new("x", BigInt::from(0));
    //     let mut coefficients: Vec<T> = Vec::new();
    //     coefficients.push(T::zero());
    //     UnivariatePolynomial::new(coefficients, variable, None)
    // }

    // fn random(_bit_length: u64) -> Self {
    //     panic!("If you want to generate random polynomials don't use this method");
    // }

    // fn random_with_bounds(_lower_bound: BigInt, _upper_bound: BigInt) -> Self {
    //     panic!("If you want to generate random polynomials don't use this method");
    // }
