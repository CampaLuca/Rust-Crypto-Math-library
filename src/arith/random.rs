use num_traits::ToPrimitive;
use rand::Rng;
use num_bigint::{BigInt, BigUint, RandomBits, RandBigInt};
use rand_distr::{Distribution, Normal};

use crate::{poly::{classes::univariate_polynomial::UnivariatePolynomial, instances::univariate_polynomial_instance::UnivariatePolynomialInstance}, variables::vars::Var, numbers::{classes::ZZ::ZZ, numbers::{Instance, Operand, Class, Random, Number}, instances::ZZ_instance::ZZinstance}, algebras::FiniteField::{classes::Zmod::Zmod, instances::Zmod_instance::ZmodInstance}};

pub fn get_random_bigint(length: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let signed: BigInt = rng.sample(RandomBits::new(length));
    signed
}

pub fn get_random_biguint(length: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let unsigned: BigUint = rng.sample(RandomBits::new(length));
    unsigned
}

pub fn get_random_bigint_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> BigInt {
    let mut rng = rand::thread_rng();

    let b = rng.gen_bigint_range(&lower_bound, &upper_bound);
    b
}


pub fn gen_from_uniform_distribution<T>(lower_bound: BigInt, upper_bound: BigInt, degree: usize) -> UnivariatePolynomialInstance<T> where T: Operand + Instance + Clone + Eq + Random + Number{
    let mut coefficients: Vec<T> = Vec::new();
    for _i in 0..(degree+1) {
        coefficients.push(T::random_with_bounds(lower_bound.clone(), upper_bound.clone()));
    }
    
    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::from(1)), None, true)

}

pub fn gen_from_range_with_modulo<T>(lower_bound: BigInt, upper_bound: BigInt, degree: usize, modulo: BigInt) -> UnivariatePolynomialInstance<ZmodInstance> where T: Random +Operand + Instance + Clone + Eq + Number{
    let field: Zmod = Zmod::new(Some(ZZ::new().new_instance(modulo)));
    
    let mut coefficients: Vec<ZmodInstance> = Vec::new();
    for _i in 0..(degree+1) {
        coefficients.push(field.apply(T::random_with_bounds(lower_bound.clone(), upper_bound.clone())));
    }
    
    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::from(1)), None, true)

}

pub fn gen_from_uniform_distribution_with_modulo<T>(bound: BigInt, degree: usize, modulo: BigInt) -> UnivariatePolynomialInstance<ZmodInstance> where T: Random +Operand + Instance + Clone + Eq + Number{
    let field: Zmod = Zmod::new(Some(ZZ::new().new_instance(modulo)));
    
    let mut coefficients: Vec<ZmodInstance> = Vec::new();
    for _i in 0..(degree+1) {
        coefficients.push(field.apply(T::random_with_bounds(-bound.clone()/2, (bound.clone()/2+1))));
    }
    
    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::from(1)), None, true)

}

pub fn gen_from_gaussian_distribution_with_modulo<T>(mu: f32, sigma: f32, degree: usize, modulo: BigInt) -> UnivariatePolynomialInstance<ZmodInstance> where T: Random +Operand + Instance + Clone + Eq + Number{
    let field: Zmod = Zmod::new(Some(ZZ::new().new_instance(modulo)));
    let normal = Normal::new(mu, sigma).unwrap();
    let v = normal.sample(&mut rand::thread_rng());
    let mut coefficients: Vec<ZmodInstance> = Vec::new();
    for _i in 0..(degree+1) {
        coefficients.push(field.apply(BigInt::from(normal.sample(&mut rand::thread_rng()).round().to_i32().unwrap())));
    }
    
    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::from(1)), None, true)

}


fn aux_cbd(number_of_coefficients: usize, eta: usize) -> UnivariatePolynomialInstance<ZZinstance> {
    let mut i: usize = 1;
    let mut p: UnivariatePolynomialInstance<ZZinstance> = gen_from_uniform_distribution::<ZZinstance>(BigInt::from(0), BigInt::from(2), number_of_coefficients-1);
    while i < eta {
        let p1: UnivariatePolynomialInstance<ZZinstance> = gen_from_uniform_distribution::<ZZinstance>(BigInt::from(0), BigInt::from(2), number_of_coefficients-1);
        p = p + p1;
        i += 1;
    }

    p
}
pub fn gen_from_centered_binomial_distribution(number_of_coefficients: usize, eta: usize) -> UnivariatePolynomialInstance<ZZinstance> {
    let a = aux_cbd(number_of_coefficients, eta);
    let b = aux_cbd(number_of_coefficients, eta);
    a - b
}

pub fn random_byte_array(length: usize) -> Vec<u8> {
    let mut byte_array: Vec<u8> = Vec::new();
    for _i in 0..length {
        byte_array.push(rand::thread_rng().gen::<u8>())
    }

    byte_array
}
