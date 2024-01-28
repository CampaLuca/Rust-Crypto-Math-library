use num_bigint::BigInt;
use num_traits::Num;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::ring_poly_pow;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::variables::vars::Var;
use std::cmp::Ordering;
/*
    Zmod INSTANCE
*/
#[derive(Clone)]
pub struct PolynomialRingInstance<T> {
    pub class: RefCell<PolynomialRing<T>>,
    pub var: Var,
    pub coefficients: Vec<T>,
    pub ntt_form: bool
}

impl<T> PartialEq for PolynomialRingInstance<T> where T: Instance + PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool {
        (self.class.clone().into_inner()) == (other.class.clone().into_inner()) && self.var == other.var && self.coefficients == other.coefficients
    }
}
impl<T> Eq for PolynomialRingInstance<T> where T: Instance + PartialEq + Clone  {}

impl<T> PolynomialRingInstance<T> where T: Instance + Clone + PartialEq + Operand + Number {
   

    pub fn degree(&self) -> usize {
        self.coefficients.len()-1
    }

    pub fn leading_coefficient(&self) -> T {
        (*self).coefficients[(*self).coefficients.len()-1].clone()
    }

    

    pub fn unwrap_from_ring(&self) -> UnivariatePolynomialInstance<T> {
        UnivariatePolynomial::new_instance(self.coefficients.clone(), self.var.clone(), None, false)
    }
}

impl<T> PolynomialRingInstance<T> where T: ClassInstance + Instance + 'static + Clone + PartialEq + Operand + Number + Display {
    pub fn inverse(self) -> PolynomialRingInstance<T> {
        self.class.clone().into_inner().inverse(self)
    }
}







// ------------- OPERATIONS ---------------------
/*
    NEGATION
*/
impl<T> std::ops::Neg for PolynomialRingInstance<T> where T: Number + Instance + Clone + PartialEq + Operand {
    type Output = PolynomialRingInstance<T>;
    fn neg(self) -> PolynomialRingInstance<T> {
        self.class.clone().into_inner().neg(self)
    }
}



// ADDITION
impl<T> std::ops::Add<PolynomialRingInstance<T>> for PolynomialRingInstance<T> where T: Number + Instance + PartialEq + Clone + Operand {
    type Output = PolynomialRingInstance<T>;
    fn add(self, rhs: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        if self.class == rhs.class {
        
            self.class.clone().into_inner().add(self, rhs)
        } else {
            panic!("[ERROR] Impossible to add polynomials in different polynomial rings")
        }
    }
}

// SUBTRACTION
impl<T> std::ops::Sub<PolynomialRingInstance<T>> for PolynomialRingInstance<T> where T: Number + Instance + PartialEq  + Clone + Operand {
    type Output = PolynomialRingInstance<T>;
    fn sub(self, rhs: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        if self.class == rhs.class {
            self.class.clone().into_inner().sub(self, rhs)
        } else {
            panic!("[ERROR] Impossible to subtract polynomials in different polynomial rings")
        }
    }
}

// MULTIPLICATION by an other polynomial in the RING
impl<T> std::ops::Mul<PolynomialRingInstance<T>> for PolynomialRingInstance<T> where T: Display + 'static + Number + Instance + PartialEq  + Clone + Operand + ClassInstance{
    type Output = PolynomialRingInstance<T>;
    fn mul(self, rhs: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        if self.class == rhs.class {
            self.class.clone().into_inner().mul(self, rhs)
        } else {
            panic!("[ERROR] Impossible to multiply polynomials in different polynomial rings")
        }
    }
}

// MULTIPLICATION by a scalar value
impl<T> std::ops::Mul<T> for PolynomialRingInstance<T> where T: Number + Instance + PartialEq  + Clone + Operand {
    type Output = PolynomialRingInstance<T>;
    fn mul(self, rhs: T) -> PolynomialRingInstance<T> {
        let coefficients = self.coefficients.into_iter().map(|x| {
            x.mul(&rhs)
        }).collect();
        self.class.clone().into_inner().new_instance(self.var, coefficients, self.ntt_form)
    }
}

// DIVISION
impl<T> std::ops::Div<PolynomialRingInstance<T>> for PolynomialRingInstance<T> where T: 'static + Display + ClassInstance + Number + Instance + PartialEq  + Clone + Operand {
    type Output = PolynomialRingInstance<T>;
    fn div(self, rhs: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        if self.class == rhs.class {
            self.class.clone().into_inner().div(self, rhs)
        } else {
            panic!("[ERROR] Impossible to multiply polynomials in different polynomial rings")
        }
    }
}





// to be corrected TODO
impl<T> num_traits::pow::Pow<BigInt> for PolynomialRingInstance<T> where T: Display + 'static + ClassInstance + Number + Instance + PartialEq + Clone + Operand {
    type Output = PolynomialRingInstance<T>;
    fn pow(self, rhs: BigInt) -> PolynomialRingInstance<T> {
        ring_poly_pow::<T>(self.clone(), rhs)
    }
}



impl<T> Operand for PolynomialRingInstance<T> where T: Instance + PartialEq + Clone + Number  + Operand{
    fn neg(&self) ->  PolynomialRingInstance<T> {
        -(*self).clone()
    }
    fn add(&self, other:  &PolynomialRingInstance<T>) ->  PolynomialRingInstance<T> {
        (*self).clone() + (*other).clone()
    }
    fn sub(&self, other:  &PolynomialRingInstance<T>) ->  PolynomialRingInstance<T> {
        (*self).clone() + (*other).clone()
    }
    fn mul(&self, other:  &PolynomialRingInstance<T>) ->  PolynomialRingInstance<T> {
        (*self).clone() + (*other).clone()
    }
    fn div(&self, other:  &PolynomialRingInstance<T>) ->  PolynomialRingInstance<T> {
        (*self).clone() + (*other).clone()
    }
    fn equal(&self, other:&Self) -> bool {
        *self == *other
    }
    fn greater_than(&self, other: &Self) -> bool {
        *self > *other
    }
    fn less_than(&self, other: &Self) -> bool {
        *self < *other
    }
   
}

impl<T> Instance for PolynomialRingInstance<T> where T: Instance + PartialEq + Clone + Operand {
    fn as_any(&self) -> &dyn std::any::Any {
        panic!("Useless method");
    }
    fn has_type(&self) -> crate::numbers::sets::Class::ClassTypes {
        crate::numbers::sets::Class::ClassTypes::PolynomialRing
    }
}



    

    // fn random(bit_length: u64) -> Self {
    //     panic!("Method not implemented");
    // }

   

    // fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
    //     panic!("Method not implemented");
    // }

    // fn zero() -> Self {
    //     panic!("Method not implemented");
    // }

    // fn is_zero(self) -> bool {
    //     panic!("Method not implemented");
    // }




// COMPARISON

// COMPARISON OPERATORS
impl<T> PartialOrd for PolynomialRingInstance<T> where T: Instance + Clone + PartialEq + Operand + Number{
    fn lt(&self, other: &Self) -> bool {
        self.degree() < other.degree()
    }

    fn ge(&self, other: &Self) -> bool {
        self.degree() >= other.degree()
    }

    fn le(&self, other: &Self) -> bool {
        self.degree() <= other.degree()
    }

   fn gt(&self, other: &Self) -> bool {
        self.degree() > other.degree()
   }

   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
    }
}

impl<T> Ord for PolynomialRingInstance<T> where T: Instance + Clone + PartialEq + Operand  + Number{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}



impl<T> std::fmt::Display for PolynomialRingInstance<T> where T: Instance + Number + Operand + Clone +  std::fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("");

        for (i, coeff) in self.coefficients.iter().enumerate() {

            if coeff.clone().is_zero() {
                continue;
            }
            let mut sign: String = format!("");
            if coeff.less_than(&(T::zero())) {
                sign = format!("");
            } else {
                sign = format!("+");
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

// impl<T> ClassInstance for PolynomialRingInstance<T> where T: Instance{
//     fn get_class(&self) -> PolynomialRing<T> {
//         self.class
//     }
// }