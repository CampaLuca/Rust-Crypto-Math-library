use num_bigint::BigInt;
use num_traits::Num;

use crate::algebras::FiniteField::classes::Zmod::Zmod;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::arith::random::get_random_bigint_with_bounds;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::instances::ZZ_instance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::Number;
use crate::numbers::sets::Class::ClassTypes;
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
pub struct UnivariatePolynomial<T> {
    pub coefficients: Vec<T>,
    pub var: Var,
    pub multiplication_algorithm: String
}

impl<T> PartialEq for UnivariatePolynomial<T> where T: Instance + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients && self.var == other.var && self.var == other.var
    }
}
impl<T> Eq for UnivariatePolynomial<T> where T: Instance + PartialEq {}

impl<T> UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq + Number {
    pub fn new(coefficients: Vec<T>, var: Var, multiplication_algorithm: Option<String>) -> UnivariatePolynomial<T> {
        if multiplication_algorithm.is_none() {
            UnivariatePolynomial { coefficients: clean::<T>(coefficients), var: var, multiplication_algorithm: String::from("Naive") }
        } else {
            UnivariatePolynomial { coefficients: clean::<T>(coefficients), var: var, multiplication_algorithm: multiplication_algorithm.unwrap() }
        }
    }

    
    pub fn round(&self) -> UnivariatePolynomial<ZZinstance> {
        let mut coefficients: Vec<ZZinstance> = Vec::new();
        for i in 0..self.degree()+1 {
            coefficients.push((self.coefficients[i].clone()).round_to_zz());
        }
        UnivariatePolynomial::new(coefficients, self.var.clone(), Some(self.multiplication_algorithm.clone()))

    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()-1
    }

    pub fn leading_coefficient(&self) -> T {
        self.coefficients[self.coefficients.len()-1].clone()
    }
    
    pub fn quotient(self, irreducible_poly: UnivariatePolynomial<T>) -> PolynomialRingInstance<T> {
        let class: PolynomialRing<T> = PolynomialRing::new(irreducible_poly);
        class.new_instance(self.var, self.coefficients)
    }

    pub fn one(v: Var) -> UnivariatePolynomial<T> {
        UnivariatePolynomial::new(vec![T::one()], v, None)
    }

    pub fn zero(v: Var) -> UnivariatePolynomial<T> {
        UnivariatePolynomial::new(vec![T::zero()], v, None)
    }
}

impl<T> std::ops::Add for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomial<T>;
    fn add(self, rhs: UnivariatePolynomial<T>) -> UnivariatePolynomial<T> {
        if self.var == rhs.var {

            let mut coeff = Vec::new();
            if self.coefficients.len() > rhs.coefficients.len() {
                for i in 0..rhs.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().add(&(rhs.coefficients[i])));
                }
                for i in rhs.coefficients.len()..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone());
                }
            } else if self.coefficients.len() < rhs.coefficients.len() {
                for i in 0..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().add(&(rhs.coefficients[i])));
                }
                for i in self.coefficients.len()..rhs.coefficients.len() {
                    coeff.push(self.coefficients[i].clone());
                }
            } else {
                for i in 0..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().add(&(rhs.coefficients[i])));
                }
            }

            UnivariatePolynomial::new(coeff, self.var.clone(), Some(self.multiplication_algorithm))
        } else {
            panic!("Cannot add these polynomials")
        }
    }
}

impl<T> std::ops::Sub for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomial<T>;
    fn sub(self, rhs: UnivariatePolynomial<T>) -> UnivariatePolynomial<T> {
        if  self.var == rhs.var {
            let mut coeff = Vec::new();
            if self.coefficients.len() > rhs.coefficients.len() {
                for i in 0..rhs.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().sub(&(rhs.coefficients[i])));
                }
                for i in rhs.coefficients.len()..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone());
                }
            } else if self.coefficients.len() < rhs.coefficients.len() {
                for i in 0..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().sub(&(rhs.coefficients[i])));
                }
                for i in self.coefficients.len()..rhs.coefficients.len() {
                    coeff.push(self.coefficients[i].clone());
                }
            } else {
                for i in 0..self.coefficients.len() {
                    coeff.push(self.coefficients[i].clone().sub(&(rhs.coefficients[i])));
                }
            }

            UnivariatePolynomial::new(coeff, self.var.clone(), Some(self.multiplication_algorithm))
        } else {
            panic!("ERROR: Cannot sub these polynomials")
        }
    }
}


impl<T> std::ops::Mul for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq + Number {
    type Output = UnivariatePolynomial<T>;
    fn mul(self, rhs: UnivariatePolynomial<T>) -> UnivariatePolynomial<T> {
        if self.var == rhs.var {
            // SCHOOLBOOK MULTIPLICATION
            //if self.multiplication_algorithm == "Naive" {
                let len = self.coefficients.len() + rhs.coefficients.len() -1;
                let mut coeff: Vec<T> = vec![T::zero(); len];
                
                // schoolbook multiplication
                for i in 0..self.coefficients.len() {
                    // perform self[i] * rhs
                    for j in 0..rhs.coefficients.len() {
                        coeff[i+j] = coeff[i+j].clone().add(&(self.coefficients[i].clone().mul(&(rhs.coefficients[j]))));
                    }
                }
                
                return UnivariatePolynomial::new(coeff, self.var.clone(), Some(self.multiplication_algorithm))
            // } else if self.multiplication_algorithm == "EvaluationMethod" {
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
}



// MULTIPLICATION by a scalar value
impl<T> std::ops::Mul<T> for UnivariatePolynomial<T> where T: Instance + PartialEq  + Clone + Operand + Number {
    type Output = UnivariatePolynomial<T>;
    fn mul(self, rhs: T) -> UnivariatePolynomial<T> {
        let coefficients = self.coefficients.into_iter().map(| x| {
            x//x.mul(&rhs)
        }).collect();
        UnivariatePolynomial::new(coefficients, self.var, Some(self.multiplication_algorithm))
    }
}







impl std::ops::Rem<BigInt> for UnivariatePolynomial<ZZinstance> {
    type Output = UnivariatePolynomial<ZmodInstance>;
    fn rem(self, rhs: BigInt) -> Self::Output {
        let field: Zmod = Zmod::new(Some(rhs));
        let coefficients = self.coefficients.into_iter().map(| x| {
            field.apply(x)
        }).collect();
        UnivariatePolynomial::new(coefficients, self.var, Some(self.multiplication_algorithm))
    }
}


impl<T> std::ops::Div for UnivariatePolynomial<T> where T: Instance + Clone + PartialEq + Operand + Number {
    type Output = (UnivariatePolynomial<T>, UnivariatePolynomial<T>);
    fn div(self, rhs: UnivariatePolynomial<T>) -> (UnivariatePolynomial<T>, UnivariatePolynomial<T>) {
        let q_and_r: Vec<UnivariatePolynomial<T>> = poly_divmod(&self.clone(), &rhs.clone());
        (q_and_r[0].clone(), q_and_r[1].clone())
    }
}


impl<T> std::fmt::Display for UnivariatePolynomial<T> where T: Instance + Operand + Clone +  Number + std::fmt::Display {
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



impl<T> Operand for UnivariatePolynomial<T> where T: Instance + Operand + Clone + Eq + Number{
    fn neg(&self) -> Self {
        let mut coefficients: Vec<T> = self.coefficients.clone();
        for i in 0..coefficients.len() {
            coefficients[i] = coefficients[i].neg();
        }

        UnivariatePolynomial::new(coefficients, self.var.clone(), Some(self.multiplication_algorithm.clone()))
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


impl<T> Instance for UnivariatePolynomial<T> where T: Instance + Clone + Operand + Eq {
    fn as_any(&self) -> &dyn std::any::Any {
        panic!("Method not implemented");
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::UnivariatePolynomial
    }
}


impl<T> Number for UnivariatePolynomial<T> where T: Instance + Clone + Number + Operand + PartialEq{
    fn is_zero(self) -> bool {
        self.coefficients.len() == 1 && self.coefficients[0] == T::zero()
    }

    fn one() -> Self {
        let variable = Var::new("x", BigInt::from(0));
        let mut coefficients: Vec<T> = Vec::new();
        coefficients.push(T::one());
        UnivariatePolynomial::new(coefficients, variable, None)
    }

    fn zero() -> Self {
        let variable = Var::new("x", BigInt::from(0));
        let mut coefficients: Vec<T> = Vec::new();
        coefficients.push(T::zero());
        UnivariatePolynomial::new(coefficients, variable, None)
    }

    fn round_to_zz(self) -> ZZinstance {
        panic!("Not implemented yet");
    }
}
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
