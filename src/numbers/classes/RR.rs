use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use std::cell::RefCell;

use crate::poly::monomial::Monomial;

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
            _ => self.new_instance(BigDecimal::from(0))
        }
    }

    fn apply_to_monomial<T: Instance>(&self, monomial: Monomial<T>) -> Monomial<RRinstance> {
        Monomial::new(monomial.variables, self.apply(monomial.coefficient))
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::RR
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




