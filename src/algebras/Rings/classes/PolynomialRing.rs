//use sagemath::numbers::sets::General_Class;

use num_traits::Num;

use crate::algebras::FiniteField::classes::Zmod::Zmod;
use crate::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::numbers::numbers::Class;
use crate::numbers::numbers::ClassInstance;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::numbers::numbers::StatefulClass;
use crate::poly::classes::univariate_polynomial::UnivariatePolynomial;
use crate::transform::ntt;
use crate::transform::ntt::NTT;
use crate::utilities::utils;
use crate::utilities::utils::clean_coefficients;
use std::cell::RefCell;
use std::fmt::Display;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::poly::instances::univariate_polynomial_instance::UnivariatePolynomialInstance;
use crate::variables::vars::Var;
// wrapper on ZZ_instance
#[derive(Clone)]
pub struct PolynomialRing<T> {
    pub irreducible_polynomial: UnivariatePolynomialInstance<T>,
    pub ntt_enabled: bool,
    pub ntt_ctxt: Option<RefCell<NTT>>
}


// impl<T> class<PolynomialRing_instance<T>> for PolynomialRing<T> where T: Instance {
//     // need to implement apply and has_type
//     fn apply(&self, value: T) -> PolynomialRing_instance<T> {
//         match value.has_type() {
//             ClassTypes::UnivariatePolynomial => self.new_instance((*value.as_any().downcast_ref::<BigInt>().unwrap()).clone()),
//             _ => self.new_instance(BigInt::from(0))
//         }
//     }


//     fn applyToMonomial(&self, monomial: Monomial<T>) -> Monomial<Zmod_instance> {
//         Monomial::new(monomial.variables, self.apply(monomial.coefficient))
//     }


//     fn has_type(&self) -> ClassTypes {
//         ClassTypes::Zmod
//     }
// }

impl<T> PolynomialRing<T> where T: Instance + Clone + PartialEq + Operand + Number + ClassInstance + 'static + Display{
    pub fn apply(&self, x: &UnivariatePolynomialInstance<T>, ntt_form: bool) -> PolynomialRingInstance<T> {
        if self.irreducible_polynomial.degree() > x.degree() {
            self.new_instance(x.var.clone(), x.coefficients.clone(), ntt_form)
        } else {
            let qr: Vec<UnivariatePolynomialInstance<T>> = utils::poly_divmod(x, &(self.irreducible_polynomial));

            self.new_instance(qr[1].var.clone(), qr[1].coefficients.clone(), ntt_form)
        }
    }
}


impl<T> PartialEq for PolynomialRing<T> where T: Instance + PartialEq + Clone{
    fn eq(&self, other: &Self) -> bool {
        self.irreducible_polynomial == other.irreducible_polynomial
    }
}
impl<T> Eq for PolynomialRing<T> where T: Instance + PartialEq + Clone {}

impl<T> PolynomialRing<T> where T: Instance + Operand + Clone + PartialEq + Number + ClassInstance + 'static{
    pub fn one(self, v: Var, generator: &Box<dyn StatefulClass>) -> PolynomialRingInstance<T> {
        self.new_instance(v, vec![generator.one().as_any().downcast_ref::<T>().unwrap().clone()], self.ntt_enabled)
    }

    pub fn zero(self, v: Var, generator: &Box<dyn StatefulClass>) -> PolynomialRingInstance<T> {
        if self.ntt_enabled {
            return self.new_instance(v, vec![generator.zero().as_any().downcast_ref::<T>().unwrap().clone(); self.irreducible_polynomial.degree()], self.ntt_enabled);
        }
        self.new_instance(v, vec![T::zero()], self.ntt_enabled)
    }
}


impl<T> PolynomialRing<T> where T: Instance + Operand + Clone + PartialEq + Number  {
    pub fn new(irreducible_polynomial: UnivariatePolynomialInstance<T>) -> PolynomialRing<T> {
        PolynomialRing { irreducible_polynomial: irreducible_polynomial, ntt_enabled: false, ntt_ctxt: None } 
    }

    pub fn get_ntt_enabled_ring(&self, ntt_ctxt: RefCell<NTT>) -> PolynomialRing<ZmodInstance> {
        let field = Zmod::new(Some(ZZ::new().new_instance(ntt_ctxt.clone().into_inner().q)));
        let mut irreducible_polynomial_coefficients: Vec<ZmodInstance> = Vec::new();
        for el in self.irreducible_polynomial.coefficients.clone() {
            irreducible_polynomial_coefficients.push(field.apply(el));
        }

        let mut ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(UnivariatePolynomial::new_instance(irreducible_polynomial_coefficients, self.irreducible_polynomial.var.clone(), None, false));
        ring.ntt_ctxt = Some(ntt_ctxt);
        ring.ntt_enabled = true;

        ring

    }


    pub fn new_instance(&self, var: Var, coefficients: Vec<T>, ntt_form: bool) -> PolynomialRingInstance<T> {
        if !ntt_form {
            return PolynomialRingInstance { class: RefCell::new(self.clone()), var: var, coefficients: clean_coefficients(coefficients), ntt_form: ntt_form } 
        } 

        return PolynomialRingInstance { class: RefCell::new(self.clone()), var: var, coefficients: coefficients, ntt_form: ntt_form } 

    }

    pub fn add(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let mut coeff = Vec::new();
        if x.coefficients.len() > y.coefficients.len() {
            for i in 0..y.coefficients.len() {
                coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
            }
            for i in y.coefficients.len()..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone());
            }
        } else if x.coefficients.len() < y.coefficients.len() {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
            }
            for i in x.coefficients.len()..y.coefficients.len() {
                coeff.push(x.coefficients[i].clone());
            }
        } else {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().add(&(y.coefficients[i])));
            }
        }

        self.new_instance(x.var, coeff, x.ntt_form && y.ntt_form) 


    }

    pub fn sub(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let mut coeff = Vec::new();
        if x.coefficients.len() > y.coefficients.len() {
            for i in 0..y.coefficients.len() {
                coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
            }
            for i in y.coefficients.len()..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone());
            }
        } else if x.coefficients.len() < y.coefficients.len() {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
            }
            for i in x.coefficients.len()..y.coefficients.len() {
                coeff.push(x.coefficients[i].clone());
            }
        } else {
            for i in 0..x.coefficients.len() {
                coeff.push(x.coefficients[i].clone().sub(&(y.coefficients[i])));
            }
        }

        self.new_instance(x.var, coeff, x.ntt_form && y.ntt_form) 
    }

    

    

    pub fn neg(&self, x: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let coefficients = x.coefficients.into_iter().map(| x| {
            x.neg()
        }).collect();

        self.new_instance(x.var, coefficients, x.ntt_form)   
    }

    
}


impl PolynomialRing<ZmodInstance> {
    pub fn apply_ntt_ctxt<T>(&self, instance: &PolynomialRingInstance<T>) -> PolynomialRingInstance<ZmodInstance> where T: Instance + Operand + Clone + PartialEq  + Number {
        if self.ntt_enabled {
            let mut coefficients = instance.coefficients.clone();
            for _i in coefficients.len()..instance.class.clone().into_inner().irreducible_polynomial.degree() {
                coefficients.push(T::zero());
            }
            let new_coefficients = self.ntt_ctxt.clone().unwrap().into_inner().to_ntt(coefficients);
            return PolynomialRing::<ZmodInstance>::new_ntt_instance(self, instance.var.clone(), new_coefficients);
        } else {
            panic!("NTT not enabled");
        }
    }

    fn new_ntt_instance(ring: &PolynomialRing<ZmodInstance>, var: Var, coefficients: Vec<ZmodInstance>) -> PolynomialRingInstance<ZmodInstance>   {
        if ring.ntt_enabled {
            let mut irreducible_polynomial : Vec<ZmodInstance> = Vec::new();
            let field = Zmod::new(Some(ZZ::new().new_instance(ring.ntt_ctxt.clone().unwrap().into_inner().q)));
            for el in ring.irreducible_polynomial.clone().coefficients {
                irreducible_polynomial.push(field.apply(el));
            }
            
            
            PolynomialRingInstance { class: RefCell::new(ring.clone()), var: var, coefficients: coefficients, ntt_form: true } 
        } else {
            panic!("NTT not enabled");
        }
    }


    pub fn from_ntt_ctxt(&self, instance: &PolynomialRingInstance<ZmodInstance>) -> PolynomialRingInstance<ZZinstance>{
        
        if instance.class.clone().into_inner().ntt_enabled && instance.ntt_form {
            let mut irreducible_polynomial: Vec<ZZinstance> = Vec::new();
            for el in instance.class.clone().into_inner().irreducible_polynomial.coefficients {
                irreducible_polynomial.push(el.value);
            }

            let new_coefficients: Vec<ZmodInstance> = instance.class.clone().into_inner().ntt_ctxt.unwrap().into_inner().from_ntt(instance.coefficients.clone());

            let mut coefficients: Vec<ZZinstance> = Vec::new();
            for el in new_coefficients  {
                coefficients.push(el.value);
            }

            coefficients = clean_coefficients(coefficients);

            let class: PolynomialRing<ZZinstance>  = PolynomialRing::new( UnivariatePolynomial::new_instance(irreducible_polynomial, instance.var.clone(), None, false));
            return class.new_instance(instance.var.clone(), coefficients, false);
        } else {
            panic!("ERROR: Polynomial is not in NTT format");
        }
    }
}


impl<T> PolynomialRing<T> where T: Instance + Operand + Clone + PartialEq + Number + ClassInstance + 'static + Display{
    pub fn div(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>) -> PolynomialRingInstance<T>  {
        x * y.inverse()
    }
    
    pub fn inverse(&self, x: PolynomialRingInstance<T>) -> PolynomialRingInstance<T> {
        let result: Vec<PolynomialRingInstance<T>> = utils::egcd(x.clone(), self.apply(&self.irreducible_polynomial,false));
        if result[0] != self.clone().one(x.var, &x.coefficients[0].get_class()) {
            panic!("The inverse does not exist");
        } else {
            return result[1].clone()
        }
    }

    fn aux_ntt_negative_convolution_multiplication(a0: T, a1: T, b0: T, b1: T, zeta: ZmodInstance) -> (T, T) {
        let mut r0 = a1.mul(&b1);
        r0 = r0.mul(&(zeta.as_any().downcast_ref::<T>().unwrap().clone()));
        r0 = r0.add(&(a0.mul(&b0)));

        let mut r1 = a0.mul(&b1);
        r1 = r1.add(&(a1.mul(&b0)));

        (r0, r1)
    }

    pub fn mul(&self, x: PolynomialRingInstance<T>, y: PolynomialRingInstance<T>)-> PolynomialRingInstance<T>  {

        if x.ntt_form && y.ntt_form && self.ntt_enabled && x.class.into_inner().irreducible_polynomial == y.class.into_inner().irreducible_polynomial {
            let mut coeff: Vec<T> = Vec::new();

            if self.irreducible_polynomial.degree() % 4 == 0 && self.irreducible_polynomial.coefficients[0].clone() == self.irreducible_polynomial.coefficients[0].clone().get_class().one().as_any().downcast_ref::<T>().unwrap().clone() {
                let q = self.ntt_ctxt.clone().unwrap().into_inner().q;
                let zetas = self.ntt_ctxt.clone().unwrap().into_inner().zetas;
                let field = Zmod::new(Some(ZZ::new().new_instance(q)));
                let index = self.irreducible_polynomial.degree()/4;
                for i in 0..index {
                    let (r0, r1) = PolynomialRing::aux_ntt_negative_convolution_multiplication(x.coefficients[4*i+0].clone(), x.coefficients[4*i+1].clone(), y.coefficients[4*i+0].clone(), y.coefficients[4*i+1].clone(), field.apply(zetas[index+i].clone()));
                    let (r2, r3) = PolynomialRing::aux_ntt_negative_convolution_multiplication(x.coefficients[4*i+2].clone(), x.coefficients[4*i+3].clone(), y.coefficients[4*i+2].clone(), y.coefficients[4*i+3].clone(), field.apply(-zetas[index+i].clone()));
                    coeff.push(r0);
                    coeff.push(r1);
                    coeff.push(r2);
                    coeff.push(r3);
                    
                }
            } else {
                
                for j in 0..x.coefficients.len() {
                    coeff.push(x.coefficients[j].clone().mul(&y.coefficients[j].clone()));
                }
            }
            self.new_instance(x.var, coeff, true)
        } else {
            
            // schoolbook multiplication, then reducing by irreducible poly thanks to divmod
            let len = x.coefficients.len() + y.coefficients.len() -1;
            let mut coeff: Vec<T> = vec![T::zero(); len];
            
            // schoolbook multiplication
            for i in 0..x.coefficients.len() {
                // perform self[i] * rhs
                for j in 0..y.coefficients.len() {
                    coeff[i+j] = coeff[i+j].clone().add(&(x.coefficients[i].clone().mul(&(y.coefficients[j]))));
                }
            }

            
            println!("Schoolbook");

            let current_poly = UnivariatePolynomial::new_instance(coeff, x.var.clone(), None, true);
            self.apply(&current_poly, false)
        }
    }
}

