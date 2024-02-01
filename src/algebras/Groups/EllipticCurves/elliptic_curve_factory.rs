use crate::numbers::{numbers::{ClassInstance, Instance, Operand}, sets::Class::ClassTypes};

use super::elliptic_curve_generic::EllipticCurve_generic;


pub trait EllipticCurve<T> {
    fn discriminant(&self) -> T;
    fn b_invariants(&self) -> (T,T,T,T);
}


#[derive(Clone)]
pub struct EllipticCurveFactory {
}


impl EllipticCurveFactory {
    pub fn gen_elliptic_curve<V,T>(coefficients: Vec<T>)  -> Box<EllipticCurve_generic<T>>  where T: 'static + Instance + Operand + PartialEq + Clone + ClassInstance, V: EllipticCurve<T> {
        if coefficients.len() == 0 {
            panic!("You should set the curve parameters");
        }

        match coefficients[0].has_type() { 
            // ClassTypes::QQ => return Box::new(EllipticCurve_rational_field::new(coefficients)),
            // ClassTypes::ZZ => return Box::new(EllipticCurve_number_field::new(coefficients)),
            // ClassTypes::RR => return Box::new(EllipticCurve_number_field::new(coefficients)),
            // ClassTypes::Zmod => return Box::new(EllipticCurve_finite_field::new(coefficients)),
            // //ClassTypes::PolynomialRing => Box::new(EllipticCurve_polynomial_ring::new(coefficients)),
            _ => Box::new(EllipticCurve_generic::new(coefficients))
        } 

    }
}


