use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::PrimitiveNumber;
use crate::numbers::numbers::StatefulClass;
use crate::numbers::sets::Class::ClassTypes;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::instances::QQ_instance::QQinstance;
use crate::numbers::instances::RR_instance::RRinstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Class;
use crate::poly;
use crate::poly::classes::monomial::Monomial;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use num_integer::Integer;
use std::cell::RefCell;
use crate::poly::instances::monomial_instance::MonomialInstance;


#[derive(Clone)]
pub struct QQ;
impl Class<QQinstance> for QQ {
    fn apply<T: Instance>(&self, value: T) -> QQinstance {
        match value.has_type() {
            ClassTypes::BigInt => {
                let one: BigInt = BigInt::one();
                self.new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone(),  one)
            },
            ClassTypes::QQ => self.new_instance((*value.as_any().downcast_ref::<QQinstance>().unwrap()).numerator.clone(),  (*value.as_any().downcast_ref::<QQinstance>().unwrap()).denominator.clone()),
            ClassTypes::ZZ => {
                let one: BigInt = BigInt::one();
                self.new_instance((*value.as_any().downcast_ref::<ZZinstance>().unwrap()).value.clone(), one)
            },
            ClassTypes::RR => self.new_instance_from_real((*value.as_any().downcast_ref::<RRinstance>().unwrap()).value.clone()),
            ClassTypes::BigDecimal => self.new_instance_from_real((*value.as_any().downcast_ref::<BigDecimal>().unwrap()).clone()),
            ClassTypes::Zmod => self.apply((*value.as_any().downcast_ref::<ZmodInstance>().unwrap()).value.clone()),
            _ => self.new_instance(BigInt::from(0), BigInt::from(1))
        }
    }

    fn apply_to_monomial<T: Instance + Number>(&self, monomial: MonomialInstance<T>) -> MonomialInstance<QQinstance> {
        Monomial::new_monomial(monomial.variables, self.apply(monomial.coefficient))
    }

    fn has_type(&self) -> ClassTypes {
        ClassTypes::QQ
    }

    fn apply_to_univariate_poly<T: Instance + Number + Operand + Clone + PartialEq>(&self, polynomial: UnivariatePolynomialInstance<T>) -> UnivariatePolynomialInstance<QQinstance> {
        let mut coefficients: Vec<QQinstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }

        UnivariatePolynomial::new_instance(coefficients, polynomial.var.clone(), polynomial.class.into_inner().multiplication_algorithm, polynomial.clean_coefficients)
    }

    fn apply_to_poly_ring<T: Instance + Number + Operand + Clone + PartialEq+ClassInstance+'static>(&self, polynomial: PolynomialRingInstance<T>) -> PolynomialRingInstance<QQinstance> {
        let mut coefficients: Vec<QQinstance> = Vec::new();
        for i in 0..polynomial.degree()+1 {
            coefficients.push(self.apply(polynomial.coefficients[i].clone()));
        }
        
        let ring = PolynomialRing::new(self.apply_to_univariate_poly(polynomial.class.clone().into_inner().irreducible_polynomial.clone()), polynomial.class.clone().into_inner().fixed_length_coefficients);

        return ring.new_instance(polynomial.var.clone(), coefficients, false);
    }
}

impl PartialEq for QQ {
    fn eq(&self, other: &Self) -> bool {
        self.has_type() == other.has_type()
    }
}
impl Eq for QQ {}

impl Copy for QQ {}

impl QQ {
    pub fn new() -> QQ {
        QQ {}
    }
    

    pub fn new_instance(&self, numerator: BigInt, denominator: BigInt) -> QQinstance {
        let new_instance = QQinstance { class: RefCell::new(*self), numerator: numerator, denominator: denominator };
        new_instance.simplify()
    }


    pub fn create_instance(&self, numerator: BigInt, denominator: BigInt) -> QQinstance {
        QQinstance { class: RefCell::new(*self), numerator: numerator, denominator: denominator }
    }

    pub fn new_instance_from_real(&self, value: BigDecimal) -> QQinstance {
        let (bigint, decimal_part_digits) = value.into_bigint_and_exponent();
        let base: BigInt = BigInt::from(10);
        self.new_instance(bigint, base.pow(decimal_part_digits as u32))
    }

    pub fn one(&self) -> QQinstance {
        QQinstance { class: RefCell::new(*self), numerator: BigInt::from(1), denominator: BigInt::from(1)}
    }

    pub fn zero(&self) -> QQinstance {
        QQinstance { class: RefCell::new(*self), numerator: BigInt::from(0), denominator: BigInt::from(1)}
    }

    pub fn add(&self, x: QQinstance, y: QQinstance) -> QQinstance {
        let mcm: BigInt = x.denominator.lcm(&y.denominator);
        let new_numerator: BigInt = BigInt::from(x.numerator*(mcm.clone() / x.denominator) + y.numerator*(mcm.clone() / y.denominator));
        self.new_instance(new_numerator, mcm)
    }

    pub fn sub(&self, x: QQinstance, y: QQinstance) -> QQinstance {
        let mcm: BigInt = x.denominator.lcm(&y.denominator);
        let new_numerator: BigInt = BigInt::from(x.numerator*(mcm.clone() / x.denominator) - y.numerator*(mcm.clone() / y.denominator));
        self.new_instance(new_numerator, mcm)
    }

    pub fn mul(&self, x: QQinstance, y: QQinstance)-> QQinstance  {
        self.new_instance(x.numerator*y.numerator, x.denominator*y.denominator)
    }


    pub fn neg(&self, x: QQinstance) -> QQinstance {
        self.new_instance(-1*x.numerator, x.denominator)
    }

    pub fn inverse(&self, x: QQinstance) -> QQinstance {
        self.new_instance(x.denominator, x.numerator)
    }
}


impl StatefulClass for QQ {
    fn one(&self) -> Box<dyn Instance> {
        Box::new(self.one())
    }

    fn zero(&self) -> Box<dyn Instance> {
        Box::new(self.zero())
    }

  

}