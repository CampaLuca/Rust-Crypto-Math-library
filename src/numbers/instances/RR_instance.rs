use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Random;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::classes::RR::RR;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use core::any::Any;
use std::cell::RefCell;
use crate::numbers::numbers::generic_pow;
use std::cmp::Ordering;

// REAL NUMBERS
#[derive(Clone)]
pub struct RRinstance {
    pub class: RefCell<RR>,
    pub value: BigDecimal
}

impl PartialEq for RRinstance {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.value == other.value
    }
}
impl Eq for RRinstance {}

// impl RR_constructor for BigDecimal {
//     fn RR(&self) -> RR_instance {
//         RR_instance::new((*self).clone())
//     }
// }

// impl RR_constructor for ZZinstance {
//     fn RR(&self) -> RR_instance {
//         RR_instance::new(BigDecimal::from(self.value.clone()))
//     }
// }

// impl RR_constructor for QQinstance {
//     fn RR(&self) -> RR_instance {
//         RR_instance::new(BigDecimal::from(self.numerator.clone())/BigDecimal::from(self.denominator.clone()) )
//     }
// }

// #[macro_export]
// macro_rules! RR {
//     ($e:expr) => { RR_constructor::RR(&$e) };
// }
// macro_rules! RR {
//     ($value: float) => {
//         RR_instance.new(General_Class.new("RR"), $value);
//     };
//     ($obj: ZZinstance) => {
//         RR_instance.new(General_Class.new("RR"), ($obj.value as f64))
//     };
//     ($obj: QQinstance) => {
//         RR_instance.new(General_Class.new("RR"), $obj.numerator / $obj.denominator)
//     }
// }

impl RRinstance {


 

    // fn Add(self, other) -> RR_instance {
    //     if other.has_type() == "RR" {
    //         RR!(self.value + other.value)
    //     } else if other.has_type() == "QQ" {
    //         self + RR!(other as QQinstance)
    //     } else if other.has_type() == "ZZ" {
    //         self + RR!(other as ZZinstance)
    //     } else {
    //         panic!("Not valid input in Addition");
    //     }
    // }

    // fn Sub(self, other) -> RR_instance {
    //     if other.has_type() == "RR" {
    //         RR!(self.value - other.value)
    //     } else if other.has_type() == "QQ" {
    //         self - RR!(other as QQinstance)
    //     } else if other.has_type() == "ZZ" {
    //         self - RR!(other as ZZinstance)
    //     } else {
    //         panic!("Not valid input in Subtraction");
    //     }
    // }

    // fn Mul(self, other) -> RR_instance {
    //     if other.has_type() == "RR" {
    //         RR!(self.value * other.value)
    //     } else if other.has_type() == "QQ" {
    //         self * RR!(other as QQinstance)
    //     } else if other.has_type() == "ZZ" {
    //         self * RR!(other as ZZinstance)
    //     } else {
    //         panic!("Not valid input in Multiplication");
    //     }
    // }

    // fn Div(self, other) -> RR_instance  {
    //     if other.has_type() == "RR" {
    //         RR!(self.value / other.value)
    //     } else if other.has_type() == "QQ" {
    //         self / RR!(other as QQinstance)
    //     } else if other.has_type() == "ZZ" {
    //         self / RR!(other as ZZinstance)
    //     } else {
    //         panic!("Not valid input in Division");
    //     }
    // }

    // fn Neg(self) -> RR_instance {
    //     RR!(-self.value)
    // }

    // fn Mod(self, other) {
    //     // to be implemented yet
    // }
}


// ------------- OPERATIONS -------------------
/*
    NEGATION
*/
impl std::ops::Neg for RRinstance {
    type Output = RRinstance;
    fn neg(self) -> RRinstance {
        self.class.clone().into_inner().apply(-self.value)
    }
}

/*
    SUM --> the trait will receive a clone of the struct and not a pointer. If we want
    to change put &ZZinstance on the header and *self before the '+' sign. Do that for all the other operations
    and the &self in the arguments
*/
impl std::ops::Add<ZZinstance> for RRinstance {
    type Output = RRinstance;
    fn add(self, rhs: ZZinstance) -> RRinstance {
        self.clone() + self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Add<QQinstance> for RRinstance {
    type Output = RRinstance;
    fn add(self, rhs: QQinstance) -> RRinstance {
        self.clone() + self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Add<RRinstance> for RRinstance {
    type Output = RRinstance;
    fn add(self, rhs: RRinstance) -> RRinstance {
        self.class.clone().into_inner().add(self,rhs)
    }
}

/*
    SUBTRACTION
*/
impl std::ops::Sub<ZZinstance> for RRinstance {
    type Output = RRinstance;
    fn sub(self, rhs: ZZinstance) -> RRinstance {
        self.clone() -self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Sub<QQinstance> for RRinstance {
    type Output = RRinstance;
    fn sub(self, rhs: QQinstance) -> RRinstance {
        self.clone() -self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Sub<RRinstance> for RRinstance {
    type Output = RRinstance;
    fn sub(self, rhs: RRinstance) -> RRinstance {
        self.class.clone().into_inner().sub(self,rhs)
    }
}

/*
    MULTIPLICATION
*/
impl std::ops::Mul<ZZinstance> for RRinstance {
    type Output = RRinstance;
    fn mul(self, rhs: ZZinstance) -> RRinstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Mul<QQinstance> for RRinstance {
    type Output = RRinstance;
    fn mul(self, rhs: QQinstance) -> RRinstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Mul<RRinstance> for RRinstance {
    type Output = RRinstance;
    fn mul(self, rhs: RRinstance) -> RRinstance {
        self.class.clone().into_inner().mul(self,rhs)
    }
}

/*
    DIVISION
*/
impl std::ops::Div<ZZinstance> for RRinstance {
    type Output = RRinstance;
    fn div(self, rhs: ZZinstance) -> RRinstance {
        self.clone() / self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Div<QQinstance> for RRinstance {
    type Output = RRinstance;
    fn div(self, rhs: QQinstance) -> RRinstance {
        self.clone() / self.class.clone().into_inner().apply(rhs).clone()
    }
}
impl std::ops::Div<RRinstance> for RRinstance {
    type Output = RRinstance;
    fn div(self, rhs: RRinstance) -> RRinstance {
        self.class.clone().into_inner().div(self,rhs)
    }
}

// to be corrected TODO
impl num_traits::pow::Pow<BigInt> for RRinstance {
    type Output = RRinstance;
    fn pow(self, rhs: BigInt) -> RRinstance {
        generic_pow::<RRinstance>(self.clone(), rhs)
    }
}

impl Instance for RRinstance {
    fn has_type(&self) -> ClassTypes {
        self.class.clone().into_inner().has_type()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Number for RRinstance {
    fn one() -> RRinstance {
        let c = RR::new();
        c.one()
    }
    fn zero() -> RRinstance {
        let c = RR::new();
        c.zero()
    }
    fn is_zero(self) -> bool {
        self == RRinstance::zero()
    }
    fn round_to_zz(self) -> ZZinstance {
        ZZ::new().apply(self)
    }
}

impl Random for RRinstance {
    fn random(bit_length: u64) -> Self {
        let value: BigDecimal = BigDecimal::random(bit_length);
        RR::new().new_instance(value)
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let value: BigDecimal = BigDecimal::random_with_bounds(lower_bound, upper_bound);
        RR::new().new_instance(value)
    }
}

impl Operand for RRinstance {
    fn neg(&self) -> RRinstance {
        -((*self).clone())
    }
    fn add(&self, other: &RRinstance) -> RRinstance {
        (*self).clone() + (*other).clone()
    }
    fn sub(&self, other: &RRinstance) -> RRinstance {
        (*self).clone() - (*other).clone()
    }
    fn mul(&self, other: &RRinstance) -> RRinstance {
        (*self).clone() * (*other).clone()
    }
    fn div(&self, other: &RRinstance) -> RRinstance {
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


// COMPARISON
impl PartialOrd for RRinstance {
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

impl Ord for RRinstance {
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

impl std::fmt::Display for RRinstance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{0}", self.value)
    }
}

