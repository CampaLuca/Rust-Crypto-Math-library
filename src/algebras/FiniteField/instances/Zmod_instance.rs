//use sagemath::numbers::sets::General_Class;
use num_bigint::BigInt;
use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::numbers::PrimitiveNumber;
use crate::numbers::numbers::Random;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use core::any::Any;
use std::cell::RefCell;
use std::cmp::Ordering;
use crate::numbers::numbers::generic_pow;
use crate::algebras::FiniteField::classes::Zmod::Zmod;

/*
    Zmod INSTANCE
*/
#[derive(Clone)]
pub struct ZmodInstance {
    pub class: RefCell<Zmod>,
    pub value: ZZinstance
}

impl PartialEq for ZmodInstance {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.value == other.value
    }
}
impl Eq for ZmodInstance {}

impl ZmodInstance {
    pub fn inverse(&self) -> ZmodInstance {
        self.class.clone().into_inner().inverse((*self).clone()) 
    }

    pub fn get_bigint_value(&self) -> ZZinstance {
        self.value.clone()
    }
}

// ------------- OPERATIONS ---------------------
/*
    NEGATION
*/
impl std::ops::Neg for ZmodInstance {
    type Output = ZmodInstance;
    fn neg(self) -> ZmodInstance {
        self.class.clone().into_inner().neg(self)
    }
}

/*
    SUM
*/
impl std::ops::Add<ZmodInstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn add(self, rhs: ZmodInstance) -> ZmodInstance {
        
        if rhs.class.clone().into_inner().module.is_none() {
            return  self.class.clone().into_inner().add(self, rhs)  
        } else if self.class.clone().into_inner().module.is_none() {
            return rhs.class.clone().into_inner().add(self, rhs)
        } else {
            if self.class == rhs.class {
                self.class.clone().into_inner().add(self, rhs)
            } else {
                panic!("The values are not in the same field")
            }
        }
    }
}

impl std::ops::Add<ZZinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn add(self, rhs: ZZinstance) -> ZmodInstance {
        self.clone() + self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Add<QQinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn add(self, rhs: QQinstance) -> ZmodInstance {
        self.clone() + self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Add<RRinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn add(self, rhs: RRinstance) -> ZmodInstance {
        self.clone() + self.class.clone().into_inner().apply(rhs)
    }
}

/*
    SUBTRACTION
*/
impl std::ops::Sub<ZmodInstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn sub(self, rhs: ZmodInstance) -> ZmodInstance {
        if rhs.class.clone().into_inner().module.is_none() {
            return  self.class.clone().into_inner().sub(self, rhs)  
        } else if self.class.clone().into_inner().module.is_none() {
            return rhs.class.clone().into_inner().sub(self, rhs)
        } else {
            if self.class == rhs.class {
                self.class.clone().into_inner().sub(self, rhs)
            } else {
                panic!("The values are not in the same field {} {}", self.class.into_inner().module.unwrap(), rhs.class.into_inner().module.unwrap());
            }
        }
    }
}
impl std::ops::Sub<ZZinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn sub(self, rhs: ZZinstance) -> ZmodInstance {
        self.clone() - self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Sub<QQinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn sub(self, rhs: QQinstance) -> ZmodInstance {
        self.clone() - self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Sub<RRinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn sub(self, rhs: RRinstance) -> ZmodInstance {
        self.clone() - self.class.clone().into_inner().apply(rhs)
    }
}

/*
    MULTIPLICATION
*/
impl std::ops::Mul<ZmodInstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn mul(self, rhs: ZmodInstance) -> ZmodInstance {
        if rhs.class.clone().into_inner().module.is_none() {
            return  self.class.clone().into_inner().mul(self, rhs)  
        } else if self.class.clone().into_inner().module.is_none() {
            return rhs.class.clone().into_inner().mul(self, rhs)
        } else {
            if self.class == rhs.class {
                self.class.clone().into_inner().mul(self, rhs)
            } else {
                panic!("The values are not in the same field")
            }
        }
    }
}
impl std::ops::Mul<ZZinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn mul(self, rhs: ZZinstance) -> ZmodInstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Mul<QQinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn mul(self, rhs: QQinstance) -> ZmodInstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Mul<RRinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn mul(self, rhs: RRinstance) -> ZmodInstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}

/*
    DIVISION
*/
impl std::ops::Div<ZmodInstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn div(self, rhs: ZmodInstance) -> ZmodInstance {
        if rhs.class.clone().into_inner().module.is_none() {
            return  self.class.clone().into_inner().div(self, rhs)  
        } else if self.class.clone().into_inner().module.is_none() {
            return rhs.class.clone().into_inner().div(self, rhs)
        } else {
            if self.class == rhs.class {
                self.class.clone().into_inner().div(self, rhs)
            } else {
                panic!("The values are not in the same field")
            }
        }
    }
}
impl std::ops::Div<ZZinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn div(self, rhs: ZZinstance) -> ZmodInstance {
        self.clone() / self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Div<QQinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn div(self, rhs: QQinstance) -> ZmodInstance {
        self.clone() / self.class.clone().into_inner().apply(rhs)
    }
}

impl std::ops::Div<RRinstance> for ZmodInstance {
    type Output = ZmodInstance;
    fn div(self, rhs: RRinstance) -> ZmodInstance {
        self.clone() / self.class.clone().into_inner().apply(rhs)
    }
}



// COMPARISON OPERATORS
impl PartialOrd for ZmodInstance {
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

   fn gt(&self, other: &Self) -> bool {
        self.value > other.value
   }

   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
    }
}

impl Ord for ZmodInstance {
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


// to be corrected TODO
impl num_traits::pow::Pow<BigInt> for ZmodInstance {
    type Output = ZmodInstance;
    fn pow(self, rhs: BigInt) -> ZmodInstance {
        generic_pow::<ZmodInstance>(self.clone(), rhs)
    }
}


impl Instance for ZmodInstance {
    fn has_type(&self) -> ClassTypes {
        self.class.clone().into_inner().has_type()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Number for ZmodInstance{
    fn one() -> ZmodInstance {
        let c = Zmod::new(Some(ZZ::new().new_instance(BigInt::from(2))));
        c.one()
    }
    fn zero() -> ZmodInstance {
        let c = Zmod::new(None);
        c.zero()
    }
    fn is_zero(self) -> bool {
        self.value == ZZinstance::zero()
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().apply(self.value)
    }
}




    
impl Random for ZmodInstance {
    fn random(bit_length: u64) -> Self {
        panic!("Method not implemented");
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        panic!("Method not implemented");
    }
}

impl Operand for ZmodInstance {
    fn neg(&self) -> ZmodInstance {
        -((*self).clone())
    }
    fn add(&self, other: &ZmodInstance) -> ZmodInstance {
        (*self).clone() + (*other).clone()
    }
    fn sub(&self, other: &ZmodInstance) -> ZmodInstance {
        (*self).clone() - (*other).clone()
    }
    fn mul(&self, other: &ZmodInstance) -> ZmodInstance {
        (*self).clone() * (*other).clone()
    }
    fn div(&self, other: &ZmodInstance) -> ZmodInstance {
        (*self).clone() / (*other).clone()
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

impl std::fmt::Display for ZmodInstance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{0}", self.value)
    }
}

impl ClassInstance for ZmodInstance {
    fn get_class(&self) -> Box<dyn StatefulClass> {
        Box::new(self.class.clone().into_inner())
    }
}