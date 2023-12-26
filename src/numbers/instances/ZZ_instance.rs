//use sagemath::numbers::sets::General_Class;
use num_bigint::BigInt;
use crate::arith::primes::is_prime;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Random;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::Class;
use core::any::Any;
use std::cell::RefCell;
use crate::numbers::numbers::generic_pow;
use std::cmp::Ordering;
use crate::arith::primes::is_pseudoprime;

// INTEGERS

#[derive(Clone)]
pub struct ZZinstance {
    pub class: RefCell<ZZ>,
    pub value: BigInt
}

impl PartialEq for ZZinstance {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.value == other.value
    }
}
impl Eq for ZZinstance {}


// impl ZZ_constructor for BigInt {
//     fn ZZ(&self) -> ZZ_instance {
//         ZZ_instance::new((*self).clone())
//     }
// }

// impl ZZ_constructor for QQ_instance {
//     fn ZZ(&self) -> ZZ_instance {
//         ZZ_instance::new(utils::round_to_bigint((BigDecimal::from(self.numerator.clone())) / (BigDecimal::from(self.denominator.clone()))))
//     }
// }

// impl ZZ_constructor for RR_instance {
//     fn ZZ(&self) -> ZZ_instance {
//         ZZ_instance::new(utils::round_to_bigint(self.value.clone()))
//     }
// }

// #[macro_export]
// macro_rules! ZZ {
//     ($e:expr) => { ZZ_constructor::ZZ(&$e) };
// }


// macro_rules! ZZ {
//     ($value: BigInt) => {
//         ZZ_instance.new(General_Class.new("ZZ"), $value);
//     };
//     ($obj: QQ_instance) => {
//         ZZ_instance.new(General_Class.new("ZZ"), BigInt::from($obj.numerator / $obj.denominator))
//     };
//     ($obj: RR_instance) => {
//         ZZ_instance.new_from_real(General_Class.new("ZZ"), BigInt::from($obj.value))
//     }
// }

// macro_rules! text_addition {
//     ($value:expr) => {
//         $value.to_string()

//         // match on input type?
//     };
// }

impl ZZinstance {
    pub fn next_prime(&self) -> ZZinstance {
        let mut starting_value = self.value.clone();
        let mut found: bool = false;

        while !found {
            starting_value = starting_value + 1;
            if is_prime(starting_value.clone().to_biguint().unwrap()) {
                found = true
            }
        }

        self.class.clone().into_inner().apply(starting_value)
    }

    pub fn next_probable_prime(&self) -> ZZinstance {
        let mut starting_value = self.value.clone();
        let mut found: bool = false;

        while !found {
            starting_value = starting_value + 1;
            if is_pseudoprime(starting_value.clone(), true) {
                found = true
            }
        }

        self.class.clone().into_inner().apply(starting_value)
    }
}


// ------------------- OPERATIONS ---------------
/*
    NEGATION
*/
impl std::ops::Neg for ZZinstance {
    type Output = ZZinstance;
    fn neg(self) -> ZZinstance {
        self.class.clone().into_inner().neg(self)
    }
}

/*
    SUM
*/
impl std::ops::Add<ZZinstance> for ZZinstance {
    type Output = ZZinstance;
    fn add(self, rhs: ZZinstance) -> ZZinstance {
        self.class.clone().into_inner().add(self, rhs)
    }
}
impl std::ops::Add<QQinstance> for ZZinstance {
    type Output = QQinstance;
    fn add(self, rhs: QQinstance) -> QQinstance {
        rhs + self
    }
}
impl std::ops::Add<RRinstance> for ZZinstance {
    type Output = RRinstance;
    fn add(self, rhs: RRinstance) -> RRinstance {
        rhs + self
    }
}

/*
    SUBTRACTION
*/
impl std::ops::Sub<ZZinstance> for ZZinstance {
    type Output = ZZinstance;
    fn sub(self, rhs: ZZinstance) -> ZZinstance {
        self.class.clone().into_inner().sub(self, rhs)
    }
}

impl std::ops::Sub<QQinstance> for ZZinstance {
    type Output = QQinstance;
    fn sub(self, rhs: QQinstance) -> QQinstance {
        (-(rhs)) + self
    }
}

impl std::ops::Sub<RRinstance> for ZZinstance {
    type Output = RRinstance;
    fn sub(self, rhs: RRinstance) -> RRinstance {
        (-(rhs)) + self
    }
}

/*
    MULTIPLICATION
*/
impl std::ops::Mul<ZZinstance> for ZZinstance {
    type Output = ZZinstance;
    fn mul(self, rhs: ZZinstance) -> ZZinstance {
        self.class.clone().into_inner().mul(self, rhs)
    }
}

impl std::ops::Mul<QQinstance> for ZZinstance {
    type Output = QQinstance;
    fn mul(self, rhs: QQinstance) -> QQinstance {
        rhs * self
    }
}

impl std::ops::Mul<RRinstance> for ZZinstance {
    type Output = RRinstance;
    fn mul(self, rhs: RRinstance) -> RRinstance {
        rhs * self
    }
}

/*
    DIVISION
*/
impl std::ops::Div<ZZinstance> for ZZinstance {
    type Output = ZZinstance;
    fn div(self, rhs: ZZinstance) -> ZZinstance {
        self.class.clone().into_inner().div(self, rhs)
    }
}

impl std::ops::Div<QQinstance> for ZZinstance {
    type Output = QQinstance;
    fn div(self, rhs: QQinstance) -> QQinstance {
        rhs.class.clone().into_inner().apply(self) / rhs
    }
}

impl std::ops::Div<RRinstance> for ZZinstance {
    type Output = RRinstance;
    fn div(self, rhs: RRinstance) -> RRinstance {
        rhs.class.clone().into_inner().apply(self) / rhs
    }
}


impl num_traits::pow::Pow<ZZinstance> for ZZinstance {
    type Output = ZZinstance;
    fn pow(self, rhs: ZZinstance) -> ZZinstance {
        generic_pow::<ZZinstance>(self, rhs.value)
    }
}

impl num_traits::pow::Pow<BigInt> for ZZinstance {
    type Output = ZZinstance;
    fn pow(self, rhs: BigInt) -> ZZinstance {
        generic_pow::<ZZinstance>(self, rhs)
    }
}

impl num_traits::pow::Pow<ZmodInstance> for ZZinstance {
    type Output = ZZinstance;
    fn pow(self, rhs: ZmodInstance) -> ZZinstance {
        generic_pow::<ZZinstance>(self, rhs.value)
    }
}



// impl num_traits::pow::Pow<QQ_instance> for ZZ_instance {
//     type Output = QQ_instance;
//     fn pow(self, rhs: QQ_instance) -> QQ_instance {
//         let v = generic_pow::<BigInt>(self, rhs.numerator);
//         // now I need to answer with the rhs.denominator-root of the value v
//     }
// }

// impl num_traits::pow::Pow<RR_instance> for ZZ_instance {
//     type Output = RR_instance;
//     fn pow(self, rhs: RR_instance) -> RR_instance {
//         let fractional = QQ::New();
//         let v = Pow::pow(self, fractional::apply(rhs));
//         rhs.class::apply(v)
//         // now I need to answer with the rhs.denominator-root of the value v
//     }
// }



impl Instance for ZZinstance {
    fn has_type(&self) -> ClassTypes {
        self.class.clone().into_inner().has_type()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Number for ZZinstance {
    fn one() -> ZZinstance {
        let c = ZZ::new();
        c.one()
    }
    fn zero() -> ZZinstance {
        let c = ZZ::new();
        c.zero()
    }
    fn is_zero(self) -> bool {
        self == ZZinstance::zero()
    }
    fn round_to_zz(self) -> ZZinstance {
        self
    }
}

impl Random for ZZinstance {
    fn random(bit_length: u64) -> Self {
        let value: BigInt = BigInt::random(bit_length);
        ZZ::new().new_instance(value)
    }

    fn random_with_bounds(lower_bound: BigInt, upper_bound: BigInt) -> Self {
        let value: BigInt = BigInt::random_with_bounds(lower_bound, upper_bound);
        ZZ::new().new_instance(value)
    }
}
impl Operand for ZZinstance {
    fn neg(&self) -> ZZinstance {
        -((*self).clone())
    }
    fn add(&self, other: &ZZinstance) -> ZZinstance {
        (*self).clone() + (*other).clone()
    }
    fn sub(&self, other: &ZZinstance) -> ZZinstance {
        (*self).clone() - (*other).clone()
    }
    fn mul(&self, other: &ZZinstance) -> ZZinstance {
        (*self).clone() * (*other).clone()
    }
    fn div(&self, other: &ZZinstance) -> ZZinstance {
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
impl PartialOrd for ZZinstance {
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

impl Ord for ZZinstance {
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

impl std::fmt::Display for ZZinstance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{0}", self.value)
    }
}