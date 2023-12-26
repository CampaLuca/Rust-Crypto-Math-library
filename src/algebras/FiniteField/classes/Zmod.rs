//use sagemath::numbers::sets::General_Class;
use num_bigint::BigInt;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::poly::univariate_polynomial::UnivariatePolynomial;
use crate::utilities::utils;

use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use bigdecimal::BigDecimal;
use crate::numbers::sets::Class::ClassTypes;
use std::cell::RefCell;
use crate::poly::monomial::Monomial;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;

// wrapper on ZZ_instance
#[derive(Clone)]
pub struct Zmod {
    pub module: Option<BigInt>
}



impl Class<ZmodInstance> for Zmod {
    // need to implement apply and has_type
    fn apply<T: Instance + Number>(&self, value: T) -> ZmodInstance {
        match value.has_type() {
            ClassTypes::BigInt => self.new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone()),
            ClassTypes::QQ => self.new_instance(utils::round_to_bigint((BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).numerator.clone())) / (BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).denominator.clone())))),
            ClassTypes::ZZ => self.new_instance((*value.as_any().downcast_ref::<ZZinstance>().unwrap()).value.clone()),
            ClassTypes::RR => self.new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<RRinstance>().unwrap()).value.clone())),
            ClassTypes::BigDecimal => self.new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<BigDecimal>().unwrap()).clone())),
            _ => self.new_instance(BigInt::from(0))
        }
    }


    fn apply_to_monomial<T: Instance + Number>(&self, monomial: Monomial<T>) -> Monomial<ZmodInstance> {
        Monomial::new(monomial.variables, self.apply(monomial.coefficient))
    }


    fn has_type(&self) -> ClassTypes {
        ClassTypes::Zmod
    }

    fn apply_to_univariate_poly<T: Instance + Number + Operand + Clone + PartialEq>(&self, polynomial: crate::poly::univariate_polynomial::UnivariatePolynomial<T>) -> crate::poly::univariate_polynomial::UnivariatePolynomial<ZmodInstance> {
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }

        UnivariatePolynomial::new(coefficients, polynomial.var.clone(), Some(polynomial.multiplication_algorithm))

    }

}


impl PartialEq for Zmod {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}
impl Eq for Zmod {}

impl Zmod {
    pub fn new(module: Option<BigInt>) -> Zmod {
        Zmod { module: module }
    }

    fn new_instance(&self, value: BigInt) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: value % self.clone().module.unwrap().clone()}
    }

    pub fn one(&self) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: BigInt::from(1) % self.clone().module.unwrap().clone()}
    }

    pub fn zero(&self) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: BigInt::from(0) % self.clone().module.unwrap().clone()}
    }

    pub fn add(self, x: ZmodInstance, y: ZmodInstance) -> ZmodInstance {
        self.apply(x.value + y.value)
    }

    pub fn sub(self, x: ZmodInstance, y: ZmodInstance) -> ZmodInstance {
        self.apply(x.value - y.value)
    }

    pub fn mul(self, x: ZmodInstance, y: ZmodInstance)-> ZmodInstance  {
        self.apply(x.value * y.value)
    }

    pub fn div(&self, x: ZmodInstance, y: ZmodInstance) -> ZmodInstance  {
        self.apply(x.value * self.clone().inverse(y).value)
    }

    pub fn neg(self, x: ZmodInstance) -> ZmodInstance {
        self.apply(-x.value)
    }

    pub fn inverse(self, x: ZmodInstance) -> ZmodInstance {
        self.apply(utils::modular_inverse(x.value.clone(), self.clone().module.unwrap()))
    }
}


