use crate::numbers::numbers::Number;
use crate::variables::vars::Var;
use num_bigint::BigInt;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use either::*;
use num_traits::Pow;
use crate::poly::polynomial::Polynomial;

// MONOMIALS
// create struct MONOMI which will compose the polynomial
#[derive(Clone)]
pub struct Monomial<T> {
    pub variables: Vec<Var>,
    pub coefficient: T //it should implement the trait
}


impl<T> Monomial<T> where T: Instance + Operand + Clone + Number {
    pub fn new(vars: Vec<Var>, coefficient: T) -> Monomial<T> {
        Monomial { variables: vars, coefficient: coefficient }
    }

    pub fn new_from_var(a: Var) -> Monomial<T> {
        // implement function
        let mut vars: Vec<Var> = Vec::new();
        vars.push(a);
        Monomial { variables: vars, coefficient: T::one()}
    }

    fn is_similar(&self, other: Monomial<T>) -> bool {
        if self.variables.len() == other.variables.len() && self.coefficient.has_type() == other.coefficient.has_type() {
            for i in 0..self.variables.len() {
                if self.variables[i] != other.variables[i] {
                    return false;
                }
            }

            return true;
        } 

        false
    }
}

impl<T> PartialEq for Monomial<T> where T: Instance + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        if self.variables.len() != other.variables.len() {
            return false;
        }

        let mut equal: bool = true;
        for i in 0..self.variables.len() {
            equal = equal && (self.variables[i] == other.variables[i]);
        }

        equal && (self.coefficient == other.coefficient)
        
    }
}
impl<T> Eq for Monomial<T> where T: Instance + PartialEq {}


// OPERATIONS

impl<T> std::ops::Neg for Monomial<T> where T: Instance + Operand + Clone + Number{
    type Output = Monomial<T>;
    fn neg(self) -> Monomial<T> {
        Monomial::new(self.variables, self.coefficient.neg())
    }
}

impl<T> std::ops::Add for Monomial<T> where T: Instance + Operand + Clone + Number {
    type Output = Either<Monomial<T>, Polynomial<T>>;
    fn add(self, rhs: Monomial<T>) -> Either<Monomial<T>, Polynomial<T>> {
        if self.is_similar(rhs.clone()) {
            let k: T = rhs.coefficient;
            Left(Monomial::new(self.variables, self.coefficient.add(&k)))
        } else {
            //create polynomial
            let mut monomials = Vec::new();
            monomials.push(self);
            monomials.push(rhs);
            Right(Polynomial::new(monomials))
        }
    }
}

impl<T> std::ops::Sub for Monomial<T> where T: Instance + Operand + Clone + Number {
    type Output = Either<Monomial<T>, Polynomial<T>>;
    fn sub(self, rhs: Monomial<T>) -> Either<Monomial<T>, Polynomial<T>> {
        if self.is_similar(rhs.clone()) {
            let k: T = rhs.coefficient;
            Left(Monomial::new(self.variables, self.coefficient.sub(&k)))
        } else {
            //create polynomial
            let mut monomials = Vec::new();
            monomials.push(self);
            monomials.push(-rhs);
            Right(Polynomial::new(monomials))
        }
    }
}


// multiplication and division will generate a new monomial
impl<T> std::ops::Mul for Monomial<T> where T: Instance + Operand + Clone + Number {
    type Output = Monomial<T>;
    fn mul(self, rhs: Monomial<T>) -> Monomial<T> {
        let k: T = rhs.coefficient;
        let mut variables_clone = self.variables.clone();
        for i in rhs.variables {
            let mut new: bool = true;
            for j in &mut variables_clone {
                if i == *j {
                    new = false;
                    *j = (*j).clone()*i.clone();
                    break;
                } else {}
            }
            if new == true {
                variables_clone.push(i)
            } else {}
        }
        Monomial::new(variables_clone, self.coefficient.mul(&k))
        
    }
}

impl<T> std::ops::Div for Monomial<T> where T: Instance + Operand + Clone + Number {
    type Output = Monomial<T>;
    fn div(self, rhs: Monomial<T>) -> Monomial<T> {
        
        let k: T = rhs.coefficient;
        let mut variables_clone = self.variables.clone();
        for i in rhs.variables {
            let mut new: bool = true;
            for j in &mut variables_clone {
                if i == *j {
                    new = false;
                    *j = (*j).clone()/i.clone();
                    break;
                } else {}
            }
            if new == true {
                variables_clone.push(i.pow(BigInt::from(-1)))
            } else {}
        }
        Monomial::new(variables_clone, self.coefficient.div(&k))
        
    }
}


impl<T> std::fmt::Display for Monomial<T> where T: Instance + Operand + Clone +  std::fmt::Display  {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("{0}", self.coefficient);
        

        for i in &self.variables {
            let variable = format!("{i}");
            printable = format!("{printable}{variable}")
        }

        write!(f, "{0}", printable )
    }
}