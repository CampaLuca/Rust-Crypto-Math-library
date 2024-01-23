use num_bigint::BigInt;

use crate::numbers::classes::RR::RR;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Random;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::classes::QQ::QQ;

use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::Operand;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use num_integer::Integer;
use core::any::Any;

use std::cmp::Ordering;
use std::cell::RefCell;


// INSTANCES
#[derive(Clone)]
pub struct QQinstance {
    pub class: RefCell<QQ>,
    pub numerator: BigInt,
    pub denominator: BigInt
}



impl PartialEq for QQinstance {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.numerator == other.numerator && self.denominator == other.denominator
    }
}
impl Eq for QQinstance {}


impl QQinstance {

    pub fn simplify(self) -> QQinstance {
        let gcd_numerator_denominator: BigInt = (self.numerator).gcd(&self.denominator);
        if gcd_numerator_denominator == BigInt::from(1) {
            return self.class.clone().into_inner().create_instance(self.numerator, self.denominator);
        }
        let num: BigInt = BigInt::from(self.numerator / gcd_numerator_denominator.clone());
        let den: BigInt = BigInt::from(self.denominator / gcd_numerator_denominator.clone());
        self.class.clone().into_inner().new_instance(num, den)
    }

    pub fn inverse(self) -> QQinstance {
        self.class.clone().into_inner().inverse(self)
    }

}

// ------------------- OPERATIONS ----------------
/*
    NEGATION
*/
impl std::ops::Neg for QQinstance {
    type Output = QQinstance;
    fn neg(self) -> QQinstance {
       self.class.clone().into_inner().neg(self)
    }
}

/*
    SUM
*/
impl std::ops::Add<ZZinstance> for QQinstance {
    type Output = QQinstance;
    fn add(self, rhs: ZZinstance) -> QQinstance {
        self.clone() + self.class.clone().into_inner().apply(rhs) 
    }
}
impl std::ops::Add<QQinstance> for QQinstance {
    type Output = QQinstance;
    fn add(self, rhs: QQinstance) -> QQinstance {
        self.class.clone().into_inner().add(self, rhs)
    }
}
impl std::ops::Add<RRinstance> for QQinstance {
    type Output = RRinstance;
    fn add(self, rhs: RRinstance) -> RRinstance {
        rhs + self
    }
}

/*
    SUBTRACTION
*/
impl std::ops::Sub<ZZinstance> for QQinstance {
    type Output = QQinstance;
    fn sub(self, rhs: ZZinstance) -> QQinstance {
        self.clone() - self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Sub<QQinstance> for QQinstance {
    type Output = QQinstance;
    fn sub(self, rhs: QQinstance) -> QQinstance {
        self.class.clone().into_inner().sub(self.clone(), rhs)
    }
}
impl std::ops::Sub<RRinstance> for QQinstance {
    type Output = RRinstance;
    fn sub(self, rhs: RRinstance) -> RRinstance {
        -(rhs) + self
    }
}

/*
    MULTIPLICATION
*/
impl std::ops::Mul<ZZinstance> for QQinstance {
    type Output = QQinstance;
    fn mul(self, rhs: ZZinstance) -> QQinstance {
        self.clone() * self.class.clone().into_inner().apply(rhs)
    }
}
impl std::ops::Mul<QQinstance> for QQinstance {
    type Output = QQinstance;
    fn mul(self, rhs: QQinstance) -> QQinstance {
        self.class.clone().into_inner().mul(self, rhs)
    }
}
impl std::ops::Mul<RRinstance> for QQinstance {
    type Output = RRinstance;
    fn mul(self, rhs: RRinstance) -> RRinstance {
        rhs * self
    }
}

// to be corrected TODO
// impl num_traits::pow::Pow<BigInt> for QQ_instance {
//     type Output = Var;
//     fn pow(self, rhs: BigInt) -> Var {
//         let mut result = self.clone();
//         result.exponent = result.exponent * rhs;
//         result
//     }
// }
/*
    DIVISION
*/
impl std::ops::Div<ZZinstance> for QQinstance {
    type Output = QQinstance;
    fn div(self, rhs: ZZinstance) -> QQinstance {
        self.clone() * (self.class.clone().into_inner().apply(rhs)).inverse()
    }
}
impl std::ops::Div<QQinstance> for QQinstance {
    type Output = QQinstance;
    fn div(self, rhs: QQinstance) -> QQinstance {
        self * rhs.inverse()
    }
}
impl std::ops::Div<RRinstance> for QQinstance {
    type Output = RRinstance;
    fn div(self, rhs: RRinstance) -> RRinstance {
        rhs.class.clone().into_inner().apply(self) / rhs
    }
}

impl Instance for QQinstance {
    fn has_type(&self) -> ClassTypes {
        self.class.clone().into_inner().has_type()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ClassInstance for QQinstance {
    fn get_class(&self) -> Box<dyn StatefulClass> {
        Box::new(self.class.clone().into_inner())
    }
}

impl Number for QQinstance {
    fn one() -> QQinstance {
        let c = QQ::new();
        c.one()
    }
    fn zero() -> QQinstance {
        let c: QQ = QQ::new();
        c.zero()
    }
    fn is_zero(self) -> bool {
        self == QQinstance::zero()
    }

    fn round_to_zz(self) -> ZZinstance {
        RR::new().apply(self).round_to_zz()
    }
}

impl Random for QQinstance {
    fn random(bit_length: u64) -> Self {
        let numerator: BigInt = BigInt::random(bit_length);
        let denominator: BigInt = BigInt::random(bit_length);
        QQ::new().create_instance(numerator, denominator)
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let numerator: BigInt = BigInt::random_with_bounds(lower_bound.clone(), upper_bound.clone());
        let denominator: BigInt = BigInt::random_with_bounds(lower_bound, upper_bound);
        QQ::new().create_instance(numerator, denominator)
    }
}


impl Operand for QQinstance {
    fn neg(&self) -> QQinstance {
        -((*self).clone())
    }
    fn add(&self, other: &QQinstance) -> QQinstance {
        (*self).clone() + (*other).clone()
    }
    fn sub(&self, other: &QQinstance) -> QQinstance {
        (*self).clone() - (*other).clone()
    }
    fn mul(&self, other: &QQinstance) -> QQinstance {
        (*self).clone() * (*other).clone()
    }
    fn div(&self, other: &QQinstance) -> QQinstance {
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

// COMPARISON OPERATORS
impl PartialOrd for QQinstance {
    fn lt(&self, other: &Self) -> bool {
        self.clone().numerator*other.clone().denominator < self.clone().denominator*other.clone().numerator
    }

    fn ge(&self, other: &Self) -> bool {
        self.clone().numerator*other.clone().denominator >= self.clone().denominator*other.clone().numerator
    }

    fn le(&self, other: &Self) -> bool {
        self.clone().numerator*other.clone().denominator <= self.clone().denominator*other.clone().numerator
    }

   fn gt(&self, other: &Self) -> bool {
        self.clone().numerator*other.clone().denominator > self.clone().denominator*other.clone().numerator
   }

   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for QQinstance {
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

impl std::fmt::Display for QQinstance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{0}/{1}", self.numerator, self.denominator)
    }
}