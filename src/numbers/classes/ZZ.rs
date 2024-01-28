//use sagemath::numbers::sets::General_Class;
use num_bigint::BigInt;
use num_bigint::RandBigInt;
use crate::numbers::instances::ZZ_instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::poly::classes::monomial::Monomial;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::monomial_instance::MonomialInstance;
use crate::utilities::utils;
use bigdecimal::BigDecimal;
use std::cell::RefCell;
use rand;

#[derive(Clone)]
pub struct ZZ;

impl Class<ZZinstance> for ZZ {
    fn apply<T: Instance>(&self, value: T) -> ZZinstance {
        match value.has_type() {
            ClassTypes::BigInt => self.new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone()),
            ClassTypes::U32 => { let value = BigInt::from((*value.as_any().downcast_ref::<u32>().unwrap()).clone()); self.new_instance(value) },
            ClassTypes::I32 => { let value = BigInt::from((*value.as_any().downcast_ref::<u32>().unwrap()).clone()); self.new_instance(value) },
            ClassTypes::QQ => self.new_instance(utils::round_to_bigint((BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).numerator.clone())) / (BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).denominator.clone())))),
            ClassTypes::ZZ => self.new_instance((*value.as_any().downcast_ref::<ZZinstance>().unwrap()).value.clone()),
            ClassTypes::RR => self.new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<RRinstance>().unwrap()).value.clone())),
            ClassTypes::BigDecimal => self.new_instance(utils::round_to_bigint((*value.as_any().downcast_ref::<BigDecimal>().unwrap()).clone())),
            _ => self.new_instance(BigInt::from(0))
        }
    }

    fn apply_to_monomial<T: Instance + Number>(&self, monomial: MonomialInstance<T>) -> MonomialInstance<ZZinstance> {
        Monomial::new_monomial(monomial.variables, self.apply(monomial.coefficient))
        
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::ZZ
    }

    fn apply_to_univariate_poly<T: Instance + Number + Operand + Clone + PartialEq>(&self, polynomial: crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance<T>) -> crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance<ZZinstance> {
        let mut coefficients: Vec<ZZinstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }

        UnivariatePolynomial::new_instance(coefficients, polynomial.var.clone(), polynomial.class.into_inner().multiplication_algorithm, polynomial.clean_coefficients)

    }
}


impl ZZ {
    pub fn new() -> ZZ {
        ZZ {}
    }
    
    pub fn new_instance(&self, value: BigInt) -> ZZinstance {
        ZZinstance { class: RefCell::new(self.clone()), value: value}
    }

    pub fn one(&self) -> ZZinstance {
        ZZinstance { class: RefCell::new(self.clone()), value: BigInt::from(1)}
    }

    pub fn zero(&self) -> ZZinstance {
        ZZinstance { class: RefCell::new(self.clone()), value: BigInt::from(0)}
    }

    pub fn add(&self, x: ZZinstance, y: ZZinstance) -> ZZinstance {
        self.apply(x.value + y.value)
    }

    pub fn sub(&self, x: ZZinstance, y: ZZinstance) -> ZZinstance {
        self.apply(x.value - y.value)
    }

    pub fn mul(&self, x: ZZinstance, y: ZZinstance)-> ZZinstance  {
        self.apply(x.value * y.value)
    }

    pub fn div(&self, x: ZZinstance, y: ZZinstance) -> ZZinstance  {
        self.apply(x.value / y.value)
    }

    pub fn neg(&self, x: ZZinstance) -> ZZinstance {
        self.apply(-x.value)
    }
}

impl PartialEq for ZZ {
    fn eq(&self, other: &Self) -> bool {
        self.has_type() == other.has_type()
    }
}
impl Eq for ZZ {}


impl ZZ {
    pub fn randint(&self, lbound: &ZZinstance, ubound: &ZZinstance) -> ZZinstance {
        let mut rng = rand::thread_rng();
        let value = rng.gen_bigint_range(&lbound.value, &ubound.value);
        self.apply(value)
    }
}


impl StatefulClass for ZZ {
    fn one(&self) -> Box<dyn Instance> {
        Box::new(self.apply(BigInt::from(1)))
    }

    fn zero(&self) -> Box<dyn Instance> {
        Box::new(self.apply(BigInt::from(0)))
    }

  

}




