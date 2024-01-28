use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use crate::{algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance, poly::instances::monomial_instance::MonomialInstance};
use crate::numbers::sets::Class::ClassTypes;
use num_bigint::{BigInt, RandomBits, ToBigInt};
use bigdecimal::BigDecimal;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;
use num_bigint::RandBigInt;
use core::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use crate::poly::classes::monomial::Monomial;

use super::{instances::ZZ_instance::ZZinstance, classes::ZZ::ZZ};

// TRAITS that should be moved in a generic folder


/*****
 * 
 * 
 *      INSTANCE traits
 * 
 * Every instance of a class object should implement those trait to supply generic functions 
 * 
 * 
 * 
 */
pub trait Instance {
    fn has_type(&self) -> ClassTypes;
    fn as_any(&self) -> &dyn Any;
}


pub trait ClassInstance {
    fn get_class(&self) -> Box<dyn StatefulClass>;
}


/****
 * 
 *  Class TRAITS
 * 
 * Every class should implement those traits cause they need to supply at least those functionality
 * 
 */

pub trait StatefulClass {
    fn zero(&self) -> Box<dyn Instance + 'static>;
    fn one(&self) -> Box<dyn Instance + 'static>;
}

pub trait StatelessClass<V> {
    fn zero() -> V;
    fn one() -> V;
    fn has_type() -> ClassTypes;
}

/***
 * 
 * Generic traits used by values which are numeric
 * 
 */




pub trait NumberInstance {
    fn one(&self) -> Self;
    fn zero(&self) -> Self;
}
pub trait Number {
    fn one() -> Self;
    fn zero() -> Self;
    fn is_zero(self) -> bool;
    fn round_to_zz(self) -> ZZinstance;
}

pub trait Random {
    fn random(bit_length: u64) -> Self;
    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self;
}

pub trait Operand {
    fn neg(&self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn less_than(&self, other: &Self) -> bool;
    fn greater_than(&self, other: &Self) -> bool;
    fn equal(&self, other:&Self) -> bool;
}

pub trait PrimitiveNumber {
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(self) -> bool;
}


pub trait Class<V> {
    fn has_type(&self) -> ClassTypes;
    fn apply<T: Instance>(&self, value: T) -> V;
    fn apply_to_monomial<T: Instance + Number>(&self, monomial: MonomialInstance<T>) -> MonomialInstance<V>;
    fn apply_to_univariate_poly<T: Instance + Number + Operand + PartialEq + Clone>(&self, polynomial: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<V>;
}






// implementing trait for general purpose numbers such as BigInt, BigDecimal and so on
/*
    BIGINT
*/
impl Instance for BigInt {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigInt
    }
    fn as_any(&self) -> &dyn Any {
        self
    }   
}


impl PrimitiveNumber for BigInt {
    fn one() -> BigInt {
        BigInt::from(1)
    }
    fn zero() -> BigInt {
        //let c: BigInt = zero::zero()
        BigInt::from(0)
    }
    fn is_zero(self) -> bool {
        self == BigInt::zero()
    }

}

impl Random for BigInt {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint(bit_length);
        b
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
        b
    }
}


/**
 * 
 *  BIGUINT
 */

impl Instance for BigUint {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigUint
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}



impl PrimitiveNumber for BigUint {
    fn one() -> BigUint {
        BigUint::from(1 as u16)
    }
    fn zero() -> BigUint {
        BigUint::from(0 as u16)
    }
    fn is_zero(self) -> bool {
        self == BigUint::zero()
    }
}

impl Random for BigUint {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_biguint(bit_length);
        b
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_biguint_range(&lower_bound.to_biguint().unwrap(), &upper_bound.to_biguint().unwrap());
        b
    }
}

// impl Instance for i32 {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn has_type(&self) -> ClassTypes {
//         ClassTypes::I32
//     }
// }

// impl Number for i32 {
//     // fn one() -> Self {
//     //     1 as i32
//     // }

//     // fn zero() -> Self {
//     //     0 as i32
//     // }

//     fn is_zero(self) -> bool {
//         self == (0 as i32)
//     }
//     fn round_to_zz(self) -> ZZinstance {
//         ZZ::new().new_instance(BigInt::from(self))
//     }
// }

// impl Random for i32 {
//     fn random(bit_length: u64) -> Self {
//         let mut rng = rand::thread_rng();
//         let signed: BigInt = rng.sample(RandomBits::new(bit_length));
//         signed.to_i32().unwrap()
//     }

//     fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
//         let mut rng = rand::thread_rng();
//         let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
//         b.to_i32().unwrap()
//     }
// }

// impl Instance for u32 {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn has_type(&self) -> ClassTypes {
//         ClassTypes::U32
//     }
// }

// impl Number for u32 {
//     // fn one() -> Self {
//     //     1 as u32
//     // }

//     // fn zero() -> Self {
//     //     0 as u32
//     // }

//     fn is_zero(self) -> bool {
//         self == (0 as u32)
//     }
//     fn round_to_zz(self) -> ZZinstance {
//         ZZ::new().new_instance(BigInt::from(self))
//     }
// }

// impl Random for u32 {
//     fn random(bit_length: u64) -> Self {
//         let mut rng = rand::thread_rng();
//         let signed: BigInt = rng.sample(RandomBits::new(bit_length));
//         signed.to_u32().unwrap()
//     }

//     fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
//         let mut rng = rand::thread_rng();
//         let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
//         b.to_u32().unwrap()
//     }
// }

// impl Instance for usize {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn has_type(&self) -> ClassTypes {
//         ClassTypes::USIZE
//     }
// }

// impl Number for usize {

//     // fn one() -> Self {
//     //     1 as usize
//     // }

//     // fn zero() -> Self {
//     //     0 as usize
//     // }

//     fn is_zero(self) -> bool {
//         self == (0 as usize)
//     }
//     fn round_to_zz(self) -> ZZinstance {
//         ZZ::new().new_instance(BigInt::from(self))
//     }
// }


// impl Random for usize {
//     fn random(bit_length: u64) -> Self {
//         let mut rng = rand::thread_rng();
//         let signed: BigInt = rng.sample(RandomBits::new(bit_length));
//         signed.to_usize().unwrap()
//     }

//     fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
//         let mut rng = rand::thread_rng();
//         let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
//         b.to_usize().unwrap()
//     }
// }


impl Instance for BigDecimal {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigDecimal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PrimitiveNumber for BigDecimal {
    fn one() -> BigDecimal {
        BigDecimal::from(1)
    }
    fn zero() -> BigDecimal {
        BigDecimal::from(0)
    }
    fn is_zero(self) -> bool {
        self == BigDecimal::zero()
    }
}

impl Random for BigDecimal {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint(bit_length);
        BigDecimal::new(b, rng.gen::<i64>() )
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
        BigDecimal::new(b, rng.gen::<i64>() )
    }

}


impl Operand for BigInt {
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other 
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }

    fn div(&self, other: &Self) -> Self {
        self / other
    }

    fn equal(&self, other:&Self) -> bool {
        self == other
    }

    fn greater_than(&self, other: &Self) -> bool {
        self > other
    }

    fn less_than(&self, other: &Self) -> bool {
        self < other
    }

    fn neg(&self) -> Self {
        -self
    }
}

// generic function for pow
pub fn generic_pow<T: Clone+Number+Operand+std::ops::Mul<T, Output = T>>(value: T, exponent: BigInt) -> T {
    let mut base = value.clone();
    let mut exp = exponent.clone();

    if exp == BigInt::from(0) {
        return T::one();
    }

    while exp.clone() & BigInt::from(1) == BigInt::from(0) {
        base = base.clone() * base;
        exp >>= 1;
    }

    if exp == BigInt::from(1) {
        return base;
    }

    let mut acc = base.clone();
    while exp.clone() > BigInt::from(1) {
        exp >>= 1;
        base = base.clone() * base;
        if exp.clone() & BigInt::from(1) == BigInt::from(1) {
            acc = acc * base.clone();
        }
    }
    acc
}


pub fn ring_poly_pow<T>(value: PolynomialRingInstance<T>, exponent: BigInt) -> PolynomialRingInstance<T> where T: Display + 'static + ClassInstance + Instance + Number + Clone + PartialEq + Operand {
    let mut base = value.clone();
    let mut exp = exponent.clone();
    let generator: Box<dyn StatefulClass> = value.coefficients[0].get_class();

    if exp == BigInt::from(0) {
        return value.class.into_inner().one(value.var, &generator);
    }

    while exp.clone() & BigInt::from(1) == BigInt::from(0) {
        base = base.clone() * base;
        exp >>= 1;
    }

    if exp == BigInt::from(1) {
        return base;
    }

    let mut acc = base.clone();
    while exp.clone() > BigInt::from(1) {
        exp >>= 1;
        base = base.clone() * base;
        if exp.clone() & BigInt::from(1) == BigInt::from(1) {
            acc = acc * base.clone();
        }
    } 
    acc
}

pub fn poly_pow<T>(value: UnivariatePolynomialInstance<T>, exponent: BigInt) -> UnivariatePolynomialInstance<T> where T: Display + Number + Instance + Clone + PartialEq + Operand {
    let mut base = value.clone();
    let mut exp = exponent.clone();

    if exp == BigInt::from(0) {
        return UnivariatePolynomialInstance::one(value.var);
    }

    while exp.clone() & BigInt::from(1) == BigInt::from(0) {
        base = base.clone() * base;
        exp >>= 1;
    }

    if exp == BigInt::from(1) {
        return base;
    }

    let mut acc = base.clone();
    while exp.clone() > BigInt::from(1) {
        exp >>= 1;
        base = base.clone() * base;
        if exp.clone() & BigInt::from(1) == BigInt::from(1) {
            acc = acc * base.clone();
        }
    }
    acc
}

