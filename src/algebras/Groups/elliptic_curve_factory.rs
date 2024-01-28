use crate::numbers::numbers::Instance;

#[derive(Clone)]
pub struct EllipticCurveFactory {
}


impl EllipticCurveFactory {
    pub fn gen_elliptic_curve<T,V>(coeff_type: T, value_type: V, coefficients: Vec<T>) where T: Instance, V: Instance {
        
    }
}