use std::cell::RefCell;

use bigdecimal::BigDecimal;
use num_traits::Signed;
use num_traits::ToPrimitive;
use sage_math::algebras::FiniteField::classes::Zmod::Zmod;
use sage_math::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use sage_math::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use sage_math::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use sage_math::arith::random;
use sage_math::arith::random::gen_from_uniform_distribution_with_modulo;
use sage_math::arith::random::get_random_bigint;
use sage_math::cryptography::asymmetric::interfaces::interfaces::LatticeBased_PKIinterface;
use sage_math::cryptography::asymmetric::interfaces::interfaces::PKIinterface;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber1024;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber512;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber768;
use sage_math::cryptography::asymmetric::primitives::rsa::RSA;
use sage_math::numbers::classes::RR::RR;
use sage_math::numbers::classes::ZZ::ZZ;
use sage_math::numbers::instances::RR_instance::RRinstance;
use sage_math::numbers::numbers::Class;
use sage_math::numbers::numbers::Operand;
use sage_math::poly::classes::monomial::Monomial;
use sage_math::poly::classes::univariate_polynomial::UnivariatePolynomial;
use sage_math::test::test_ZZ;
use sage_math::test::test_RR;
use sage_math::test::test_QQ;
use sage_math::transform::ntt::NTTFactory;
use sage_math::transform::ntt::NTT_Algorithm;
use sage_math::transform::ntt::NTT;
use sage_math::variables::vars::Var;
use num_bigint::BigInt;
use sage_math::numbers::instances::ZZ_instance::ZZinstance;
use sage_math::poly::instances::monomial_instance::MonomialInstance;
use num_traits::Pow;


fn test_ntt() {

    //let q = Some(BigInt::from(3329));

    for i in 2..3 {
        let N: usize = 2.pow(i as u32) as usize;
        println!("N: {}", N);

        //let (q, _k) = NTT::generate_ntt_prime(N, 20, false);
        let q = Some(BigInt::from(3329));
        let zeta = NTT::get_nth_root_of_unity(q.clone().unwrap(), N);
        //let zeta = BigInt::from(17);
        println!("Zeta: {} ", zeta);
        //println!("Okay {} {}", k.clone().unwrap(), zeta);
        let ntt_ctxt = NTTFactory::init(N.clone(), q.clone().unwrap(), zeta, NTT_Algorithm::Iterative);

        let zz = ZZ::new();

        // TEST 1: test if intt(ntt(poly)) == poly
        let mut f: Vec<ZZinstance> = Vec::new();

        for _j in 0..N {
            f.push(zz.randint(&zz.new_instance(BigInt::from(0)), &zz.new_instance(q.clone().unwrap()-1)));
        }

        let f_ = ntt_ctxt.to_ntt(f.clone());
        let f2_ = ntt_ctxt.from_ntt(f_);
        
        for j in 0..N {
            if f[j].value != f2_[j].value.value {
                panic!("NTT test not passed");
            }
        }


        // TEST 2: addition
        let field: Zmod = Zmod::new(Some(zz.new_instance(q.clone().unwrap())));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one().neg());
        for _i in 1..N {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());

        let irreducible_polynomial = UnivariatePolynomial::new_instance(coefficients, var, None, false);
        let mut ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

        let mut vect: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _j in 0..2 {
            vect.push(gen_from_uniform_distribution_with_modulo::<ZZinstance>(BigInt::from(0), q.clone().unwrap(), N-1, q.clone().unwrap()).quotient(irreducible_polynomial.clone(), false))
        }

        let ntt_ring = ring.get_ntt_enabled_ring(RefCell::new(ntt_ctxt));
        let mut ntt_polys: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for j in 0..2 {
            ntt_polys.push(ntt_ring.apply_ntt_ctxt(&vect[j]));
        }
       
        // println!("{}", vect[0].clone()+vect[1].clone());
        // println!("{}", ntt_ring.from_ntt_ctxt(&(ntt_polys[0].clone()+ntt_polys[1].clone())));
        
        println!("{}", vect[0].clone());
        println!();
        println!("{}", vect[1].clone());
        // // println!("{}", q.unwrap().clone());
        // //println!("{}", vect[0].clone()*vect[1].clone());
        // println!("Done");
        println!("{}", ntt_ring.from_ntt_ctxt(&(ntt_polys[0].clone()*ntt_polys[1].clone())));
        // TEST 3: multiplication
        

    }


}

fn main() {
    test_ZZ::test();
    test_RR::test();
    test_QQ::test();
    
    // let v: Var = Var::new("x", BigInt::from(3));
    // let w: Var = Var::new("x", BigInt::from(4));
    // let m: MonomialInstance<ZZinstance> = Monomial::new_from_var(v.clone());
    // let n: MonomialInstance<ZZinstance> = Monomial::new_from_var(w.clone());
    // println!("{0}",Pow::pow(w.clone(), BigInt::from(-3))*v);

    
    // let n: usize = 8;
    //     let q: ZZinstance = ZZ::new().new_instance(BigInt::from(23));
    // let field: Zmod = Zmod::new(Some(q.clone()));
    //     let var: Var = Var::new("x", BigInt::from(1));
    //     let mut coefficients: Vec<ZmodInstance> = Vec::new();
    //     coefficients.push(field.one());
    //     for _i in 1..n {
    //         coefficients.push(field.zero());
    //     }
    //     coefficients.push(field.one());
    let cipher: Kyber1024 = Kyber1024::init();
    let plain = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18];
    let (u, v) = cipher.encrypt(plain);
 
    println!("{:?}",cipher.decrypt(u, v));


    let r_class: RR = RR::new();
    let factor: RRinstance = r_class.new_instance(BigDecimal::from(254)/BigDecimal::from(3329));
    // let irreducible_polynomial = UnivariatePolynomial::new_instance(coefficients, var, None);
    // let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

    // let a: PolynomialRingInstance<ZmodInstance> = (random::gen_from_centered_binomial_distribution(n, 2) % q.clone()).quotient(irreducible_polynomial.clone());
    // let b: PolynomialRingInstance<ZmodInstance> = (random::gen_from_centered_binomial_distribution(n, 2) % q.clone()).quotient(irreducible_polynomial.clone());

    // println!("{}",a);
    // println!("{}",b);
    
    // let c = a*b;
    // println!("{}",c);

    // let rsa: RSA = RSA::init(120);
    // println!("{}", rsa.private_keys[0].0);
    // println!("{}", rsa.private_keys[0].1);
    // println!("{}", rsa.private_keys[0].2);
    // println!("{}", rsa.public_keys[0].1);

    // println!("{}", (BigInt::from(-2).modpow(&BigInt::from(1), &BigInt::from(5))));

    // let plain = vec![1,2,3,4,5,6,7];
    // let ciphertext = rsa.encrypt(plain);

    // let plaintext = rsa.decrypt(ciphertext.clone());
    // println!("{:?}",ciphertext);
    // println!("{:?}",plaintext);
    
   //test_ntt();
}