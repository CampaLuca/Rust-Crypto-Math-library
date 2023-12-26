use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::sets::Class::ClassTypes;
use crate::poly::univariate_polynomial::UnivariatePolynomial;
use num_bigint::{BigInt, RandomBits, ToBigInt};
use bigdecimal::BigDecimal;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;
use num_bigint::RandBigInt;
use core::any::Any;
use crate::poly::monomial::Monomial;

use super::{instances::ZZ_instance::ZZinstance, classes::ZZ::ZZ};

// TRAITS that should be moved in a generic folder

pub trait Instance {
    fn has_type(&self) -> ClassTypes;
    fn as_any(&self) -> &dyn Any;
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

pub trait Class<V> {
    fn has_type(&self) -> ClassTypes;
    fn apply<T: Instance + Number>(&self, value: T) -> V;
    fn apply_to_monomial<T: Instance + Number>(&self, monomial: Monomial<T>) -> Monomial<V>;
    fn apply_to_univariate_poly<T: Instance + Number + Operand + PartialEq + Clone>(&self, polynomial: UnivariatePolynomial<T>) -> UnivariatePolynomial<V>;
}




// implementing trait for general purpose numbers such as BigInt, BigDecimal and so on

impl Instance for BigInt {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigInt
    }
    fn as_any(&self) -> &dyn Any {
        self
    }   
}

impl Number for BigInt {
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
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().new_instance(self)
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

impl Instance for BigUint {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigUint
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Number for BigUint {
    fn one() -> BigUint {
        BigUint::from(1 as u16)
    }
    fn zero() -> BigUint {
        BigUint::from(0 as u16)
    }
    fn is_zero(self) -> bool {
        self == BigUint::zero()
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().new_instance(self.to_bigint().unwrap())
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

impl Instance for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::I32
    }
}

impl Number for i32 {
    fn one() -> Self {
        1 as i32
    }

    fn zero() -> Self {
        0 as i32
    }

    fn is_zero(self) -> bool {
        self == (0 as i32)
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().new_instance(BigInt::from(self))
    }
}

impl Random for i32 {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let signed: BigInt = rng.sample(RandomBits::new(bit_length));
        signed.to_i32().unwrap()
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
        b.to_i32().unwrap()
    }
}

impl Instance for u32 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::U32
    }
}

impl Number for u32 {
    fn one() -> Self {
        1 as u32
    }

    fn zero() -> Self {
        0 as u32
    }

    fn is_zero(self) -> bool {
        self == (0 as u32)
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().new_instance(BigInt::from(self))
    }
}

impl Random for u32 {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let signed: BigInt = rng.sample(RandomBits::new(bit_length));
        signed.to_u32().unwrap()
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
        b.to_u32().unwrap()
    }
}

impl Instance for usize {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::USIZE
    }
}

impl Number for usize {

    fn one() -> Self {
        1 as usize
    }

    fn zero() -> Self {
        0 as usize
    }

    fn is_zero(self) -> bool {
        self == (0 as usize)
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().new_instance(BigInt::from(self))
    }
}


impl Random for usize {
    fn random(bit_length: u64) -> Self {
        let mut rng = rand::thread_rng();
        let signed: BigInt = rng.sample(RandomBits::new(bit_length));
        signed.to_usize().unwrap()
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let mut rng = rand::thread_rng();
        let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
        b.to_usize().unwrap()
    }
}


impl Instance for BigDecimal {
    fn has_type(&self) -> ClassTypes {
        ClassTypes::BigDecimal
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Number for BigDecimal {
    fn one() -> BigDecimal {
        BigDecimal::from(1)
    }
    fn zero() -> BigDecimal {
        BigDecimal::from(0)
    }
    fn is_zero(self) -> bool {
        self == BigDecimal::zero()
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().apply(self)
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

pub fn ring_poly_pow<T>(value: PolynomialRingInstance<T>, exponent: BigInt) -> PolynomialRingInstance<T> where T: Instance + Number + Clone + PartialEq + Operand {
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

pub fn poly_pow<T>(value: UnivariatePolynomial<T>, exponent: BigInt) -> UnivariatePolynomial<T> where T: Number + Instance + Clone + PartialEq + Operand {
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

