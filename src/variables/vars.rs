use num_bigint::BigInt;
use crate::numbers::sets::Class::ClassTypes;




#[derive(Clone)]
pub struct Var {
    types: ClassTypes,
    symbol: &'static str,
    exponent: BigInt,
    //neg: bool,
}


impl Var {
    pub fn new(symbol: &'static str, exponent: BigInt) -> Var {
        // Var {types: ClassTypes::RR, symbol: symbol, exponent: exponent, neg: false}
        Var {types: ClassTypes::RR, symbol: symbol, exponent: exponent}
    } 

    //modify the variables inside
    pub fn assume(&mut self, types: ClassTypes) {
        self.types = types;
    }

  
}


impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        // the equality is given by the common symbol and data type
        self.types == other.types && self.symbol == other.symbol && self.exponent == other.exponent
    }
}
impl Eq for Var   {}


// impl std::ops::Neg for Var {
//     type Output = Var;
//     fn neg(self) -> Var {
//         let mut new_var = self.clone();
//         new_var.neg = !self.neg;
//         new_var
//     }
// }


impl std::ops::Mul<Var> for Var  {
    type Output = Var;
    fn mul(self, rhs: Var) -> Var {
        let mut result = self.clone();
        result.exponent = result.exponent + rhs.exponent;
        result
    }
}

impl std::ops::Div<Var> for Var {
    type Output = Var;
    fn div(self, rhs: Var) -> Var{
        let mut result = self.clone();
        result.exponent = result.exponent - rhs.exponent;
        result
    }
}

impl num_traits::pow::Pow<BigInt> for Var {
    type Output = Var;
    fn pow(self, rhs: BigInt) -> Var {
        let mut result = self.clone();
        result.exponent = result.exponent * rhs;
        result
    }
}

impl std::fmt::Display for Var {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        
        write!(f, "{0}^{1}", self.symbol, self.exponent)
    }
}