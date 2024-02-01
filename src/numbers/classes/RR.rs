use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::poly::classes::monomial::Monomial;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use std::cell::RefCell;

use crate::poly::instances::monomial_instance::MonomialInstance;

#[derive(Clone)]
pub struct RR;

impl Class<RRinstance> for RR {
    fn apply<T: Instance>(&self, value: T) -> RRinstance {
        match value.has_type() {
            ClassTypes::BigInt => self.new_instance(BigDecimal::from((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone())),
            ClassTypes::QQ => self.new_instance(BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).numerator.clone())/BigDecimal::from((*value.as_any().downcast_ref::<QQinstance>().unwrap()).denominator.clone())),
            ClassTypes::ZZ => self.new_instance(BigDecimal::from((*value.as_any().downcast_ref::<ZZinstance>().unwrap()).value.clone())),
            ClassTypes::RR => self.new_instance((*value.as_any().downcast_ref::<RRinstance>().unwrap()).value.clone()),
            ClassTypes::BigDecimal => self.new_instance((*value.as_any().downcast_ref::<BigDecimal>().unwrap()).clone()),
            ClassTypes::Zmod => self.apply((*value.as_any().downcast_ref::<ZmodInstance>().unwrap()).value.clone()),
            _ => self.new_instance(BigDecimal::from(0))
        }
    }

    fn apply_to_monomial<T: Instance + Number>(&self, monomial: MonomialInstance<T>) -> MonomialInstance<RRinstance> {
        Monomial::new_monomial(monomial.variables, self.apply(monomial.coefficient))
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::RR
    }

    fn apply_to_univariate_poly<T: Instance + Number + Operand + Clone + PartialEq>(&self, polynomial: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<RRinstance> {
        let mut coefficients: Vec<RRinstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }


        UnivariatePolynomial::new_instance(coefficients, polynomial.var.clone(), polynomial.class.into_inner().multiplication_algorithm, polynomial.clean_coefficients)

    }

    fn apply_to_poly_ring<T: Instance + Number + Operand + Clone + PartialEq+ClassInstance+'static>(&self, polynomial: PolynomialRingInstance<T>) -> PolynomialRingInstance<RRinstance> {
        let mut coefficients: Vec<RRinstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }
        
        let ring = PolynomialRing::new(self.apply_to_univariate_poly(polynomial.class.clone().into_inner().irreducible_polynomial.clone()), polynomial.class.clone().into_inner().fixed_length_coefficients);

        return ring.new_instance(polynomial.var.clone(), coefficients, false);
    }
}

impl PartialEq for RR {
    fn eq(&self, other: &Self) -> bool {
        self.has_type() == other.has_type()
    }
}
impl Eq for RR {}


impl RR {
    pub fn new() -> RR {
        RR {}
    }
    
    pub fn new_instance(&self, value: BigDecimal) -> RRinstance {
        RRinstance { class: RefCell::new(self.clone()), value: value}
    }

    pub fn one(&self) -> RRinstance {
        RRinstance { class: RefCell::new(self.clone()), value: BigDecimal::from(1)}
    }

    pub fn zero(&self) -> RRinstance {
        RRinstance { class: RefCell::new(self.clone()), value: BigDecimal::from(0)}
    }

    pub fn add(&self, x: RRinstance, y: RRinstance) -> RRinstance {
        self.apply(x.value + y.value)
    }

    pub fn sub(&self, x: RRinstance, y: RRinstance) -> RRinstance {
        self.apply(x.value - y.value)
    }

    pub fn mul(&self, x: RRinstance, y: RRinstance)-> RRinstance  {
        self.apply(x.value * y.value)
    }

    pub fn div(&self, x: RRinstance, y: RRinstance) -> RRinstance  {
        self.apply(x.value / y.value)
    }

   
}


impl StatefulClass for RR {
    fn one(&self) -> Box<dyn Instance> {
        Box::new(self.apply(BigDecimal::from(1)))
    }

    fn zero(&self) -> Box<dyn Instance> {
        Box::new(self.apply(BigDecimal::from(0)))
    }

   

}

