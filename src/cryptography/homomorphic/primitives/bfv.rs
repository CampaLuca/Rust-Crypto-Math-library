use std::{cell::RefCell, fmt::Display, rc};

use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint, ToBigUint};
use num_traits::{Pow, Signed, ToPrimitive};
use crate::{algebras::{Rings::{instances::PolynomialRing_instance::{self, PolynomialRingInstance}, classes::PolynomialRing::PolynomialRing}, FiniteField::{instances::Zmod_instance::ZmodInstance, classes::Zmod::Zmod}}, arith::random::{gen_from_gaussian_distribution_with_modulo, gen_from_range_with_modulo, gen_from_uniform_distribution_with_modulo, random_byte_array}, cryptography::{asymmetric::interfaces::interfaces::{PKIinterface, KEMinterface, LatticeBased_PKIinterface}, homomorphic::interfaces::interfaces::{ BFV_PKI, FHE}}, matrices::{matrix::Matrix, vector::Vector}, numbers::{numbers::{Class, ClassInstance, Instance, Number, Operand, PrimitiveNumber, StatefulClass}, instances::{ZZ_instance::{ZZinstance, self}, RR_instance::RRinstance}, classes::RR::RR}, poly::{instances::univariate_polynomial_instance::UnivariatePolynomialInstance, classes::univariate_polynomial::UnivariatePolynomial}, transform::ntt::{NTTFactory, NTT_Algorithm, NTT}, variables::vars::Var};
use crate::arith::random::gen_from_centered_binomial_distribution;
use crate::numbers::classes::ZZ::ZZ;


// kyber utilities
fn compress<T>(poly: UnivariatePolynomialInstance<T>, modulo: ZZinstance, d: usize) -> UnivariatePolynomialInstance<ZmodInstance> where T: Display + Instance + Clone + Eq + Operand + Number {
    let q1: ZZinstance =  ZZ::new().new_instance(BigInt::from(2)).pow(BigInt::from(d));
    let r_class: RR = RR::new();
    let factor: RRinstance = r_class.new_instance(BigDecimal::from(q1.value.clone())/BigDecimal::from(modulo.value));
   
    let mut new_poly = (r_class.apply_to_univariate_poly(poly) * factor).round(); //% q1; // values are yet modulo q
    new_poly % q1
}


fn decompress<T>(poly: UnivariatePolynomialInstance<T>, modulo: ZZinstance, d: usize) -> UnivariatePolynomialInstance<ZmodInstance> where T: Instance + Clone + Eq + Operand + Number {
    let q1: ZZinstance =  ZZ::new().new_instance(BigInt::from(2)).pow(BigInt::from(d));
    let r_class: RR = RR::new();
    let factor: RRinstance = r_class.new_instance(BigDecimal::from(modulo.value.clone())/BigDecimal::from(q1.value.clone()));
    let mut new_poly = (r_class.apply_to_univariate_poly(poly) * factor).round() % modulo; // values are yet modulo q
    new_poly
}

fn plaintext_to_poly_from_bigint(plaintext: BigInt, size: usize)  -> UnivariatePolynomialInstance<ZZinstance> {
    let class = ZZ::new();
    let mut coefficients: Vec<ZZinstance> = Vec::new();
    for i in 0..size {
        coefficients.push(class.new_instance((plaintext.clone()>>i) & BigInt::from(1)))
    }

    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::one()), None, false)

}


fn poly_to_bigint(poly: PolynomialRingInstance<ZmodInstance>, threshold_prime: BigInt) -> BigInt {
    let number_of_bytes = (poly.degree()+1)/8;
    let mut plaintext: Vec<u8> = Vec::new();
    let mut threshold: BigInt = BigInt::from(2);
    if threshold_prime.clone() > BigInt::from(2) {
        threshold = (threshold_prime.clone()+1)>>1;
    }

    let mut accumulator: BigInt = BigInt::from(0u8);
    for i in (0..poly.coefficients.len()) {

            //plaintext.push(poly.coefficients[i*8+bit].value.value.to_u8().unwrap() & 0x1);
            let bit_value = 0;
            let mut coeff: BigInt;



            if poly.coefficients[i].clone().value.value.clone() >= threshold.clone()  {
                coeff = -(threshold_prime.clone()-poly.coefficients[i].clone().value.value.clone());
            } else {
                coeff = poly.coefficients[i].clone().value.value.clone();
            }

            accumulator = accumulator + (((coeff.clone()) * (BigInt::from(1) << (i)) ));
        
        
    }

    accumulator
}

fn plaintext_to_poly(plaintext: Vec<u8>, size: usize) -> UnivariatePolynomialInstance<ZZinstance> {
    let class = ZZ::new();
    let mut coefficients: Vec<ZZinstance> = Vec::new();
    for el in plaintext.clone() {
        for bit in 0..8 {
            coefficients.push(class.new_instance(BigInt::from(el & 1<<(7-bit))>>(7-bit)));
        }    
    }

    for i in (plaintext.len()*8)..size {
       
            coefficients.push(class.new_instance(BigInt::from(0)));
        
    }
    coefficients.reverse();
    UnivariatePolynomial::new_instance(coefficients, Var::new("x", BigInt::one()), None, false)
}


fn bigint_to_vecu8(value: BigInt) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for byte in 0..value.bits()/8 {
        let mut value: u8 = 0;
        for bit in 0..8 {
            value += value >> (byte*8+bit)<<bit;
        }

        vec.push(value);
    }

    vec
}


fn poly_to_plaintext(poly: PolynomialRingInstance<ZmodInstance>, threshold_prime: BigInt) -> Vec<u8> {
    let number_of_bytes = (poly.degree()+1)/8;
    let mut plaintext: Vec<u8> = Vec::new();
    let mut threshold: BigInt = BigInt::from(2);
    if threshold_prime.clone() > BigInt::from(2) {
        threshold = (threshold_prime.clone()+1)>>1;
    }

    let mut accumulator: BigInt = BigInt::from(0u8);
    for i in (0..poly.coefficients.len()).rev() {

            //plaintext.push(poly.coefficients[i*8+bit].value.value.to_u8().unwrap() & 0x1);
            let bit_value = 0;
            let mut coeff: BigInt;

            if poly.coefficients[i].clone().value.value.clone() >= threshold {
                coeff = -(threshold_prime.clone()-poly.coefficients[i].clone().value.value.clone())
            } else {
                coeff = poly.coefficients[i].clone().value.value.clone();
            }

            accumulator = accumulator + (((coeff.clone()) * (BigInt::from(1) << (i)) ));
        
        
    }

    plaintext.push(accumulator.to_u8().unwrap());
    plaintext

//VERIONE 2
    // let number_of_bytes = (poly.degree()+1)/8;
    // let mut plaintext: Vec<u8> = Vec::new();
    // let mut threshold: BigInt = BigInt::from(2);
    // if threshold_prime.clone() > BigInt::from(2) {
    //     threshold = (threshold_prime.clone()+1)>>1;
    // }

    // let mut accumulator: BigInt = BigInt::from(0);
    // println!("{}", poly);
    // for i in 0..number_of_bytes {
    //    // println!("BYte: {}", i);
    //     for bit in 0..8 {
    //         // println!("Bit: {}", bit);
    //         // println!("Value: {}", poly.coefficients[i*8+bit].clone().value.value.to_u8().unwrap());
    //         // //plaintext.push(poly.coefficients[i*8+bit].value.value.to_u8().unwrap() & 0x1);
    //         let bit_value = 0;
    //         let mut coeff: BigInt;
    //         if poly.coefficients[i*8+bit].clone().value.value.clone() >= threshold {
    //             coeff = -(threshold_prime.clone()-poly.coefficients[i*8+bit].clone().value.value.clone())
    //         } else {
    //             coeff = poly.coefficients[i*8+bit].clone().value.value.clone();
    //         }

    //         //println!("{}", accumulator);
    //         println!("Coeff: {}", coeff);
    //         accumulator = accumulator + (coeff) * (1 << (7-bit));
            
    //     }

    //     println!("byte {}: {}", i, accumulator);
    //     //plaintext.push(accumulator.to_u8().unwrap());
        
    // }

    
    // plaintext
}

pub struct BFV {
    n: usize,
    public_keys: Vec<(PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)>, // n, e
    private_keys: Vec<PolynomialRingInstance<ZmodInstance>>,// p, q, d
    primary_key: usize,
    ring: PolynomialRing<ZmodInstance>,
    q: Zmod,
    mu: f32,
    sigma: f32,
    p: Zmod,
    enabled_base_decomposition: bool
}

impl BFV {
    pub fn init(N: usize, p: BigInt, mu: f32, sigma: f32, base_decomp: bool, q_bit_length: usize) -> BFV {
        
        let (q, _k) = NTT::generate_ntt_prime(N, q_bit_length, false);
        let zeta: BigInt = NTT::get_nth_root_of_unity(q.clone().unwrap(), N);
        let ntt_ctxt = NTTFactory::init(N.clone(), q.clone().unwrap(), zeta, NTT_Algorithm::NegativeConvolution);

        // creating the RING x^N-1
        let field: Zmod = Zmod::new(Some(ZZ::new().new_instance(q.unwrap().clone())));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one());
        for _i in 1..N {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());

        let irreducible_polynomial = UnivariatePolynomial::new_instance(coefficients, var, None, false);
        let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone(), false);
        let ntt_ring = ring.get_ntt_enabled_ring(RefCell::new(ntt_ctxt));

        let mut bfv = BFV {
            n: N,
            public_keys: Vec::new(),
            private_keys: Vec::new(),
            primary_key: 0,
            ring: ntt_ring,
            q: field,
            mu: mu,
            sigma: sigma,
            p: Zmod::new(Some(ZZ::new().new_instance(p.clone()))),
            enabled_base_decomposition: base_decomp
        };

        bfv.key_gen();

        bfv

    }

    pub fn relinearization_keygen_with_base_decomposition(&self, base: f64) -> Vec<(PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)> {
        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let l = q.to_f64().unwrap().log(base).trunc().to_i64().unwrap();
        let SK = self.private_keys[self.primary_key].clone();
        let SK_square = SK.clone()*SK.clone();

        let mut RK: Vec<(PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)>  = Vec::new();

        for i in 0..(l+1) {
            let a0_i = self.ring.apply_ntt_ctxt(&(gen_from_uniform_distribution_with_modulo::<ZZinstance>(q.clone(),self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false)));
            let e_i = self.ring.apply_ntt_ctxt(&(gen_from_gaussian_distribution_with_modulo::<ZZinstance>(self.mu,self.sigma, self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false)));
         
            // let temp_sk_square = SK_square.clone() * self.q.apply(base.to_u64().unwrap().pow(i as u32));
            let temp_sk_square = self.ring.apply_ntt_ctxt(&(self.ring.from_ntt_ctxt(&SK_square, true) * ZZ::new().new_instance(BigInt::from(base.to_u64().unwrap().pow(i as u32))) % self.q.clone().module.unwrap()));
            let rk_i0 = temp_sk_square -(a0_i.clone()*SK.clone()+e_i);
            let rk_i1 = a0_i;

            RK.push((rk_i0, rk_i1));
        }

        RK
    }

    pub fn relinearization_keygen(&self, kk: BigInt) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>) {
        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let module = q*kk.clone();
        let a = (gen_from_uniform_distribution_with_modulo::<ZZinstance>(module.clone(),self.n-1, module.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false));
        let e = (gen_from_gaussian_distribution_with_modulo::<ZZinstance>(self.mu,self.sigma, self.n-1, module.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false));
        
        let r_class: RR = RR::new();
        let delta = r_class.apply(kk.clone());

        let SK1 = r_class.apply_to_poly_ring(self.ring.from_ntt_ctxt(&self.private_keys[self.primary_key].clone() , true));
        let e1 = r_class.apply_to_poly_ring(e);
        let a0 = r_class.apply_to_poly_ring(a);
        let SK2 = SK1.clone()*SK1.clone();
        let RK1_0 = a0.clone()*SK1 + e1;
        let RK1_1 = SK2*delta;
        let RK1 = RK1_1-RK1_0;
        let RK2 = a0;

        return (RK1.round() % ZZ::new().new_instance(module.clone()), RK2.round() % ZZ::new().new_instance(module));
    }

    pub fn naive_homomorphic_multiplication(&self, C1: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>), C2: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>) {
        let (a1, b1) = C1;
        let (a2, b2) = C2;

        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let p: BigInt = self.p.clone().module.unwrap().value.clone();
        let r_class: RR = RR::new();
        let delta = r_class.new_instance(BigDecimal::from(p.clone())/BigDecimal::from(q.clone()));

        let a1_tilde = r_class.apply_to_poly_ring(a1);
        let a2_tilde = r_class.apply_to_poly_ring(a2);
        let b1_tilde = r_class.apply_to_poly_ring(b1);
        let b2_tilde = r_class.apply_to_poly_ring(b2);


        let C1 = a1_tilde.clone()*a2_tilde.clone();
        let C2 = a1_tilde.clone()*b2_tilde.clone() + a2_tilde.clone()*b1_tilde.clone();
        let C3 = b1_tilde.clone()*b2_tilde;

        let mut c1_tilde = (C1* delta.clone()).round() % self.q.module.clone().unwrap();
        let mut c2_tilde = (C2 * delta.clone()).round() % self.q.module.clone().unwrap();
        let mut c3_tilde = (C3* delta).round() % self.q.module.clone().unwrap(); // values are yet modulo q

        // returned values are not in ntt context
        
      
        return (c1_tilde , c2_tilde , c3_tilde )
    }
}



impl BFV_PKI for BFV {
    fn key_gen(&mut self) {
        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let SK: PolynomialRingInstance<ZmodInstance> = self.ring.apply_ntt_ctxt(&((gen_from_uniform_distribution_with_modulo::<ZZinstance>(BigInt::from(2),self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));
        

        let PK2 = self.ring.apply_ntt_ctxt(&((gen_from_uniform_distribution_with_modulo::<ZZinstance>(q.clone(),self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));
        
        let e = self.ring.apply_ntt_ctxt(&((gen_from_gaussian_distribution_with_modulo::<ZZinstance>(self.mu,self.sigma, self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));
        
        let PK1 = -(PK2.clone()*SK.clone() +e);


        self.private_keys.push(SK.clone());
        self.public_keys.push((PK1, PK2));
    
    }

    fn decrypt(&self, ciphertext: Vec<PolynomialRingInstance<ZmodInstance>>) -> BigInt {
        // receives values not in ntt context
        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let p: BigInt = self.p.clone().module.unwrap().value.clone();

        if ciphertext.len() == 2 { // this is the relinearization key decryption
            let a = self.ring.apply_ntt_ctxt(&ciphertext[0]);
            let b = self.ring.apply_ntt_ctxt(&ciphertext[1]);
            
            let r_class: RR = RR::new();
            let delta = r_class.new_instance(BigDecimal::from(p.clone())/BigDecimal::from(q));

            let tmp = b*self.private_keys[self.primary_key].clone() + a;
            let tmp2 = self.ring.from_ntt_ctxt(&tmp, true);
            let mut new_poly = (r_class.apply_to_poly_ring(tmp2)* delta).round(); //% q1; // values are yet modulo q
            let tmp_ciphertext = (new_poly % ZZ::new().new_instance(p.clone()));
            // return poly_to_plaintext(tmp_ciphertext, p.clone());
            return poly_to_bigint(tmp_ciphertext, p.clone());


        } else if ciphertext.len() == 3 { // naive decryption
            let c1 = self.ring.apply_ntt_ctxt(&ciphertext[0]);
            let c2 = self.ring.apply_ntt_ctxt(&ciphertext[1]);
            let c3 = self.ring.apply_ntt_ctxt(&ciphertext[2]);
            
            let r_class: RR = RR::new();
            let delta = r_class.new_instance(BigDecimal::from(p.clone())/BigDecimal::from(q.clone()));
            let SK = self.private_keys[self.primary_key].clone();
            let tmp = c1 + c2*SK.clone() + (c3*SK.clone())*SK;
   
            let tmp2 = self.ring.from_ntt_ctxt(&tmp, true);

            let mut new_poly = (r_class.apply_to_poly_ring(tmp2)* delta).round(); //% q1; // values are yet modulo q
            let tmp_ciphertext = (new_poly % ZZ::new().new_instance(p.clone()));
            // return poly_to_plaintext(tmp_ciphertext, p.clone());
            
            return poly_to_bigint(tmp_ciphertext, p.clone());

        } else {
            panic!("Wrong number of values in the ciphertext");
        }
    }

    fn encrypt(&self, plaintext: BigInt) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>) {
        let q: BigInt = self.q.clone().module.unwrap().value.clone();
        let p: BigInt = self.p.clone().module.unwrap().value.clone();



        if plaintext.bits() > p.clone().to_u64().unwrap() {
            panic!("Plaintext to big. Only {} bytes can be encrypted", self.n/8);
        }

      
        // let plaintext_poly: UnivariatePolynomialInstance<ZmodInstance> = plaintext_to_poly(plaintext, self.n) % ZZ::new().new_instance(q.clone());
        let plaintext_poly: UnivariatePolynomialInstance<ZmodInstance> = plaintext_to_poly_from_bigint(plaintext, self.n) % ZZ::new().new_instance(q.clone());
        let M = ((plaintext_poly.quotient(self.ring.irreducible_polynomial.clone(), true, true)));

        let (PK1, PK2) = self.public_keys[self.primary_key].clone();

        let u: PolynomialRingInstance<ZmodInstance> = self.ring.apply_ntt_ctxt(&((gen_from_uniform_distribution_with_modulo::<ZZinstance>(BigInt::from(2),self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));
                
        let e1 = self.ring.apply_ntt_ctxt(&((gen_from_gaussian_distribution_with_modulo::<ZZinstance>(self.mu,self.sigma, self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));
        let e2 = self.ring.apply_ntt_ctxt(&((gen_from_gaussian_distribution_with_modulo::<ZZinstance>(self.mu,self.sigma, self.n-1, q.clone()).quotient(self.ring.irreducible_polynomial.clone(), true, false))));

        let delta = q.clone()/p.clone();

        let a = PK1*u.clone() + e1 + self.ring.apply_ntt_ctxt(&(M*self.q.apply(delta)));
        let b = PK2*u + e2;
      
        (self.ring.from_ntt_ctxt(&a, self.ring.fixed_length_coefficients) % ZZ::new().new_instance(q.clone()) , self.ring.from_ntt_ctxt(&b, self.ring.fixed_length_coefficients) % ZZ::new().new_instance(q.clone()))
    }
}





impl FHE<(PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)> for BFV {
    fn homomorphic_addition(&self, C1: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>), C2: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>) {
        let q: BigInt = self.q.clone().module.unwrap().value.clone();

        let (a1, b1) = C1.clone();
        let (a2, b2) = C2.clone();
        let a1_tilde = self.ring.apply_ntt_ctxt(&a1);
        let b1_tilde = self.ring.apply_ntt_ctxt(&b1);
        let a2_tilde = self.ring.apply_ntt_ctxt(&a2);
        let b2_tilde = self.ring.apply_ntt_ctxt(&b2);
        let a = a1_tilde+a2_tilde;
        let b = b1_tilde+b2_tilde;

        return (self.ring.from_ntt_ctxt(&a, self.ring.fixed_length_coefficients) % ZZ::new().new_instance(q.clone()) , self.ring.from_ntt_ctxt(&b, self.ring.fixed_length_coefficients) % ZZ::new().new_instance(q.clone()))
    }

    fn homomorphic_multiplication(&self, C1: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>), C2: (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>)) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>) {
        
        if self.enabled_base_decomposition {
            let base: f64 = 256.0;
            let RK = self.relinearization_keygen_with_base_decomposition(base);
            let (C1, C2, C3) = self.naive_homomorphic_multiplication(C1, C2);
            let q: BigInt = self.q.clone().module.unwrap().value.clone();
            let l = q.to_f64().unwrap().log(base).trunc().to_i64().unwrap();

            let zz = ZZ::new();
            let C3_decomposed = zz.apply_to_poly_ring(C3.clone()).base_decomposition(base, q.clone());

            let generator = self.q.apply(BigInt::from(0)).get_class();

            let mut C3_0 = self.ring.apply_ntt_ctxt(&self.ring.clone().zero(C3.var.clone(),  &generator)); 
            let mut C3_1 = self.ring.apply_ntt_ctxt(&self.ring.clone().zero(C3.var, &generator)); 

            for i in 0..(l+1) {
                let (rk_i0, rk_i1) = RK[i as usize].clone();
                C3_0 = C3_0 + (rk_i0*self.ring.apply_ntt_ctxt(&C3_decomposed[i as usize]));
                C3_1 = C3_1 + (rk_i1*self.ring.apply_ntt_ctxt(&C3_decomposed[i as usize]));
            }

            let a = C3_0 + self.ring.apply_ntt_ctxt(&C1);
            let b = C3_1 + self.ring.apply_ntt_ctxt(&C2);

            (self.ring.from_ntt_ctxt(&a, true) % self.q.module.clone().unwrap(), self.ring.from_ntt_ctxt(&b, true) % self.q.module.clone().unwrap())


        } else {
            
            let q: BigInt = self.q.clone().module.unwrap().value.clone();
            
            let kk: BigInt = q.clone().pow(3 as u32)+1;

            println!("{} : {}", q.clone(), kk.clone());
            let r_class: RR = RR::new();
            let delta = r_class.new_instance(BigDecimal::from(1)/BigDecimal::from(kk.clone()));
            let (RK1, RK2) = self.relinearization_keygen(kk.clone());
            let (C1, C2, C3) = self.naive_homomorphic_multiplication(C1, C2);

            let rk1 = r_class.apply_to_poly_ring(RK1);
            let rk2 = r_class.apply_to_poly_ring(RK2);

            let c3_tilde = r_class.apply_to_poly_ring(C3);

            let a = self.ring.apply_ntt_ctxt(&((c3_tilde.clone()*rk1*delta.clone()).round() % ZZ::new().new_instance(q.clone()))) + self.ring.apply_ntt_ctxt(&C1);
            let b = self.ring.apply_ntt_ctxt(&((c3_tilde*rk2*delta).round() % ZZ::new().new_instance(q.clone()))) + self.ring.apply_ntt_ctxt(&C2);
            return (self.ring.from_ntt_ctxt(&a, true) % ZZ::new().new_instance(q.clone()),self.ring.from_ntt_ctxt(&b, true) % ZZ::new().new_instance(q.clone()))
        
        }
    }
}