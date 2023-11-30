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
fn clean<T>(mut coeff: Vec<T>) -> Vec<T> where T: Instance + Operand + Clone {
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

impl<T> UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq {
    pub fn new(coefficients: Vec<T>, var: Var, multiplication_algorithm: Option<String>) -> UnivariatePolynomial<T> {
        if multiplication_algorithm.is_none() {
            UnivariatePolynomial { coefficients: clean::<T>(coefficients), var: var, multiplication_algorithm: String::from("Naive") }
        } else {
            UnivariatePolynomial { coefficients: clean::<T>(coefficients), var: var, multiplication_algorithm: multiplication_algorithm.unwrap() }
        }
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

impl<T> std::ops::Add for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq {
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

impl<T> std::ops::Sub for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq {
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


impl<T> std::ops::Mul for UnivariatePolynomial<T> where T: Instance + Operand + Clone + PartialEq {
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
impl<T> std::ops::Mul<T> for UnivariatePolynomial<T> where T: Instance + PartialEq  + Clone + Operand {
    type Output = UnivariatePolynomial<T>;
    fn mul(self, rhs: T) -> UnivariatePolynomial<T> {
        let coefficients = self.coefficients.into_iter().map(| x| {
            x.mul(&rhs)
        }).collect();
        UnivariatePolynomial::new(coefficients, self.var, Some(self.multiplication_algorithm))
    }
}


impl<T> std::fmt::Display for UnivariatePolynomial<T> where T: Instance + Operand + Clone +  std::fmt::Display {
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