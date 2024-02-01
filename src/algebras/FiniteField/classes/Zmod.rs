//use sagemath::numbers::sets::General_Class;
use num_bigint::BigInt;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::StatefulClass;
use crate::poly::classes::monomial::Monomial;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use crate::utilities::utils;

use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use bigdecimal::BigDecimal;
use crate::numbers::sets::Class::ClassTypes;
use std::cell::RefCell;
use crate::poly::instances::monomial_instance::MonomialInstance;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;

// wrapper on ZZ_instance
#[derive(Clone)]
pub struct Zmod {
    pub module: Option<ZZinstance>
}



impl Class<ZmodInstance> for Zmod {
    // need to implement apply and has_type
    fn apply<T: Instance>(&self, value: T) -> ZmodInstance {
        match value.has_type() {
            ClassTypes::BigInt => self.new_instance(ZZ::new().new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone())),
            ClassTypes::QQ => self.new_instance(ZZ::new().new_instance(utils::round_to_bigint((BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).numerator.clone())) / (BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).denominator.clone()))))),
            ClassTypes::ZZ => self.new_instance(ZZ::new().new_instance((*value.as_any().downcast_ref::<ZZinstance>().unwrap()).value.clone())),
            ClassTypes::RR => self.new_instance(ZZ::new().new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<RRinstance>().unwrap()).value.clone()))),
            ClassTypes::BigDecimal => self.new_instance(ZZ::new().new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<BigDecimal>().unwrap()).clone()))),
            ClassTypes::Zmod => self.apply((*value.as_any().downcast_ref::<ZmodInstance>().unwrap()).value.clone()),
            _ => self.new_instance(ZZ::new().new_instance(BigInt::from(0)))
        }
    }


    fn apply_to_monomial<T: Instance + Number>(&self, monomial: MonomialInstance<T>) -> MonomialInstance<ZmodInstance> {
        Monomial::new_monomial(monomial.variables, self.apply(monomial.coefficient))
    }


    fn has_type(&self) -> ClassTypes {
        ClassTypes::Zmod
    }

    fn apply_to_univariate_poly<T: Instance + Number + Operand + Clone + PartialEq>(&self, polynomial: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<ZmodInstance> {
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }

        UnivariatePolynomial::new_instance(coefficients, polynomial.var.clone(), polynomial.class.into_inner().multiplication_algorithm, polynomial.clean_coefficients)

    }

    fn apply_to_poly_ring<T: Instance + Number + Operand + Clone + PartialEq+ClassInstance+'static>(&self, polynomial: PolynomialRingInstance<T>) -> PolynomialRingInstance<ZmodInstance> {
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }
        
        let ring = PolynomialRing::new(self.apply_to_univariate_poly(polynomial.class.clone().into_inner().irreducible_polynomial.clone()), polynomial.class.clone().into_inner().fixed_length_coefficients);

        return ring.new_instance(polynomial.var.clone(), coefficients, false);
    }

}


impl PartialEq for Zmod {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}
impl Eq for Zmod {}

impl Zmod {
    pub fn new(module: Option<ZZinstance>) -> Zmod {
        Zmod { module: module }
    }

    fn new_instance(&self, value: ZZinstance) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: ZZ::new().new_instance(value.value.modpow(&BigInt::from(1), &self.module.clone().unwrap().value)) }
    }

    pub fn one(&self) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: ZZinstance::one()}
    }

    pub fn zero(&self) -> ZmodInstance {
        ZmodInstance { class: RefCell::new(self.clone()), value: ZZinstance::zero()}
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
        self.apply(utils::modular_inverse(x.value.value.clone(), self.clone().module.unwrap().value))
    }
}


impl StatefulClass for Zmod {
    fn zero(&self) -> Box<dyn Instance> {
        Box::new(self.zero())
    }

    fn one(&self) -> Box<dyn Instance> {
        Box::new(self.one())
    }

   
}

