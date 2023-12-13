use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::univariate_polynomial::UnivariatePolynomial;
use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use num_bigint::BigUint;

use core::any::Any;
use crate::poly::monomial::Monomial;


pub trait Instance {
    fn has_type(&self) -> ClassTypes;
    fn as_any(&self) -> &dyn Any;
    fn one() -> Self;
    fn zero() -> Self;
    fn is_zero(self) -> bool;
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
    // fn has_type(&self) -> ClassTypes;
}

pub trait Class<V> {
    fn has_type(&self) -> ClassTypes;
    fn apply<T: Instance>(&self, value: T) -> V;
    fn apply_to_monomial<T: Instance>(&self, monomial: Monomial<T>) -> Monomial<V>;
}

impl Instance for BigInt {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigInt
    }
    fn as_any(&self) -> &dyn Any {
        self
    }   
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

impl Instance for BigUint {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigUint
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
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

impl Instance for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::I32
    }

    fn one() -> Self {
        1 as i32
    }

    fn zero() -> Self {
        0 as i32
    }

    fn is_zero(self) -> bool {
        self == (0 as i32)
    }
}

impl Instance for u32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::U32
    }

    fn one() -> Self {
        1 as u32
    }

    fn zero() -> Self {
        0 as u32
    }

    fn is_zero(self) -> bool {
        self == (0 as u32)
    }
}

impl Instance for usize {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::USIZE
    }

    fn one() -> Self {
        1 as usize
    }

    fn zero() -> Self {
        0 as usize
    }

    fn is_zero(self) -> bool {
        self == (0 as usize)
    }
}


impl Instance for BigDecimal {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigDecimal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
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


// generic function for pow
pub fn generic_pow<T: Clone+Instance+Operand+std::ops::Mul<T, Output = T>>(value: T, exponent: BigInt) -> T {
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

pub fn ring_poly_pow<T>(value: PolynomialRingInstance<T>, exponent: BigInt) -> PolynomialRingInstance<T> where T: Instance + Clone + PartialEq + Operand {
    let mut base = value.clone();
    let mut exp = exponent.clone();

    if exp == BigInt::from(0) {
        return value.class.into_inner().one(value.var);
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

pub fn poly_pow<T>(value: UnivariatePolynomial<T>, exponent: BigInt) -> UnivariatePolynomial<T> where T: Instance + Clone + PartialEq + Operand {
    let mut base = value.clone();
    let mut exp = exponent.clone();

    if exp == BigInt::from(0) {
        return UnivariatePolynomial::one(value.var);
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

