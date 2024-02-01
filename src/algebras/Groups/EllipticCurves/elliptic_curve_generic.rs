use num_bigint::BigInt;

use crate::numbers::{classes::ZZ::ZZ, numbers::{ClassInstance, Instance, Operand}};

use super::elliptic_curve_factory::EllipticCurve;

#[derive(Clone)]
pub struct EllipticCurve_generic<T> {
    a_invariants: (T, T, T, T, T)
}


impl<T> EllipticCurve_generic<T> where T: Instance + Operand + PartialEq + ClassInstance + Clone + 'static{

    /*
    Construct an elliptic curve from Weierstrass `a`-coefficients.
     */
    pub fn new(weierstrass_coefficients: Vec<T>) -> EllipticCurve_generic<T> {
        if weierstrass_coefficients.len() != 5 {
            panic!("Wrong number of coefficients");
        }
        let curve = EllipticCurve_generic { a_invariants: (weierstrass_coefficients[0].clone(), weierstrass_coefficients[1].clone(), weierstrass_coefficients[2].clone(), weierstrass_coefficients[3].clone(), weierstrass_coefficients[4].clone()) };
        if curve.discriminant() == weierstrass_coefficients[0].get_class().zero().as_any().downcast_ref::<T>().unwrap().clone() {
            panic!("It is a singular curve. The discriminant is ZERO");
        }

        curve
    }
}


impl<T> EllipticCurve<T> for EllipticCurve_generic<T> where T: Instance + Operand + PartialEq + Clone + 'static{
    fn discriminant(&self) -> T  {
        let (b2, b4, b6, b8) = self.b_invariants();
        return b2.clone().mul(&b2).mul(&b8).neg().sub(&b4.clone().mul(&b4).mul(&b4).mul(&ZZ::new().new_instance(BigInt::from(8)).as_any().downcast_ref::<T>().unwrap())).sub(&b6.clone().mul(&b6).mul(&ZZ::new().new_instance(BigInt::from(27)).as_any().downcast_ref::<T>().unwrap())).add(&b2.clone().mul(&b4).mul(&b6).mul(ZZ::new().new_instance(BigInt::from(9)).as_any().downcast_ref::<T>().unwrap()));
    }

    fn b_invariants(&self) -> (T, T, T, T)  {
        let (a1, a2, a3, a4, a6) = self.a_invariants.clone();
        return (
            (a1.clone().mul(&a1)).add(&(a2.clone().mul(&ZZ::new().new_instance(BigInt::from(4)).as_any().downcast_ref::<T>().unwrap()))),
            (a1.clone().mul(&a3)).add(&(a4.clone().mul(&ZZ::new().new_instance(BigInt::from(2)).as_any().downcast_ref::<T>().unwrap()))),
            (a3.clone().mul(&a3)).add(&(a6.clone().mul(&ZZ::new().new_instance(BigInt::from(4)).as_any().downcast_ref::<T>().unwrap()))),
            (a1.clone().mul(&a1).mul(&a6)).add(&a2.clone().mul(&a6).mul(&(ZZ::new().new_instance(BigInt::from(4)).as_any().downcast_ref::<T>().unwrap()))).sub(&a1.clone().mul(&a3).mul(&a4)).add(&a2.clone().mul(&a3).mul(&a3)).sub(&a4.clone().mul(&a4))
        )
    }
}


