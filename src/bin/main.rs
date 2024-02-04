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
use sage_math::arith::random::get_random_bigint_with_bounds;
use sage_math::arith::random::random_byte_array;
use sage_math::cryptography::asymmetric::interfaces::interfaces::LatticeBased_PKIinterface;
use sage_math::cryptography::asymmetric::interfaces::interfaces::PKIinterface;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber1024;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber512;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber768;
use sage_math::cryptography::asymmetric::primitives::rsa::RSA;
use sage_math::cryptography::homomorphic::interfaces::interfaces::BFV_PKI;
use sage_math::cryptography::homomorphic::interfaces::interfaces::FHE;
use sage_math::cryptography::homomorphic::primitives::bfv::BFV;
use sage_math::cryptography::padding::padding::Paddings;
use sage_math::cryptography::symmetric::interfaces::interfaces::AESfactory;
use sage_math::cryptography::symmetric::modes::modes::Modes;
use sage_math::cryptography::symmetric::primitives::aes::aes_factory;
use sage_math::cryptography::symmetric::primitives::aes::AES;
use sage_math::cryptography::symmetric::primitives::aes::AES_KEY_SIZE;
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


// fn test_ntt() {

//     //let q = Some(BigInt::from(3329));

//     for i in 2..3 {
//         let N: usize = 2.pow(i as u32) as usize;
//         println!("N: {}", N);

//         //let (q, _k) = NTT::generate_ntt_prime(N, 20, false);
//         let q = Some(BigInt::from(3329));
//         let zeta: BigInt = NTT::get_nth_root_of_unity(q.clone().unwrap(), N);
//         //let zeta = BigInt::from(17);
//         println!("Zeta: {} ", zeta);
//         //println!("Okay {} {}", k.clone().unwrap(), zeta);
//         let ntt_ctxt = NTTFactory::init(N.clone(), q.clone().unwrap(), zeta, NTT_Algorithm::Iterative);

//         let zz = ZZ::new();

//         // TEST 1: test if intt(ntt(poly)) == poly
//         let mut f: Vec<ZZinstance> = Vec::new();

//         for _j in 0..N {
//             f.push(zz.randint(&zz.new_instance(BigInt::from(0)), &zz.new_instance(q.clone().unwrap()-1)));
//         }

//         let f_ = ntt_ctxt.to_ntt(f.clone());
//         let f2_ = ntt_ctxt.from_ntt(f_);
        
//         for j in 0..N {
//             if f[j].value != f2_[j].value.value {
//                 panic!("NTT test not passed");
//             }
//         }


//         // TEST 2: addition
//         let field: Zmod = Zmod::new(Some(zz.new_instance(q.clone().unwrap())));
//         let var: Var = Var::new("x", BigInt::from(1));
//         let mut coefficients: Vec<ZmodInstance> = Vec::new();
//         coefficients.push(field.one().neg());
//         for _i in 1..N {
//             coefficients.push(field.zero());
//         }
//         coefficients.push(field.one());

//         let irreducible_polynomial = UnivariatePolynomial::new_instance(coefficients, var, None, false);
//         let mut ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

//         let mut vect: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
//         for _j in 0..2 {
//             vect.push(gen_from_uniform_distribution_with_modulo::<ZZinstance>(BigInt::from(0), q.clone().unwrap(), N-1, q.clone().unwrap()).quotient(irreducible_polynomial.clone(), false))
//         }

//         let ntt_ring = ring.get_ntt_enabled_ring(RefCell::new(ntt_ctxt));
//         let mut ntt_polys: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
//         for j in 0..2 {
//             ntt_polys.push(ntt_ring.apply_ntt_ctxt(&vect[j]));
//         }
       
//         // println!("{}", vect[0].clone()+vect[1].clone());
//         // println!("{}", ntt_ring.from_ntt_ctxt(&(ntt_polys[0].clone()+ntt_polys[1].clone())));
        
//         println!("{}", vect[0].clone());
//         println!();
//         println!("{}", vect[1].clone());
//         // // println!("{}", q.unwrap().clone());
//         // //println!("{}", vect[0].clone()*vect[1].clone());
//         // println!("Done");
//         println!("{}", ntt_ring.from_ntt_ctxt(&(ntt_polys[0].clone()*ntt_polys[1].clone())));
//         // TEST 3: multiplication
        

//     }


// }

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
    // let cipher: Kyber512 = Kyber512::init();
    // let plain = vec![0,0,0,1];
    // let plain2 = vec![0, 0, 0, 1];
    // let plain = get_random_bigint_with_bounds(BigInt::from(0), BigInt::from(2).pow(32u32));
    // let plain2 = get_random_bigint_with_bounds(BigInt::from(0), BigInt::from(2).pow(32u32));
    // // let (u, v) = cipher.encrypt(plain);
 
    // // println!("{:?}",cipher.decrypt(u, v));
    // println!("V1: {}", plain.clone());
    // println!("V2: {}", plain2.clone());
    

    // let bfv: BFV = BFV::init(256, BigInt::from(32), 0.0, 1.0, true, 60);

    // let c1 = bfv.encrypt(plain);
    // let c2 = bfv.encrypt(plain2);

    // println!("Encryption Done");


    // let p1 = bfv.decrypt(vec![c1.0.clone(), c1.1.clone()]);
    // println!("{:?}", p1);
    // let p2 = bfv.decrypt(vec![c2.0.clone(),c2.1.clone()]);
    // println!("{:?}", p2);

    // println!("Decryption done");

    // // let c3 = bfv.homomorphic_addition(c1, c2);
    // // let p3 = bfv.decrypt(vec![c3.0.clone(), c3.1.clone()]);
    // // println!("{:?}", p3);
    // let c3 = bfv.homomorphic_multiplication(c1, c2);
    // let p3 = bfv.decrypt(vec![c3.0.clone(), c3.1.clone()]);
    // println!("{:?}", p3);
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

    // test_kyber();
    // test_bfv();
    // test_rsa();
    simple_aes_test();
    aes_ctr_preprocessing_test();
    aes_cbc_test();
    aes_ecb_test();
}

fn test_bfv() {

    println!("BFV test");
    let p = 32;
    let N = 1024;
    let bfv: BFV = BFV::init(N, BigInt::from(p), 0.0, 1.0, true, 60);

    let plain1 = get_random_bigint_with_bounds(BigInt::from(0), BigInt::from(2).pow(p as u32));
    let plain2 = get_random_bigint_with_bounds(BigInt::from(0), BigInt::from(2).pow(p as u32));
    let c1 = bfv.encrypt(plain1.clone());
    let c2 = bfv.encrypt(plain2.clone());



    let p1 = bfv.decrypt(vec![c1.0.clone(), c1.1.clone()]);
    assert_eq!(p1, plain1);
    let p2 = bfv.decrypt(vec![c2.0.clone(),c2.1.clone()]);
    assert_eq!(p2, plain2);


    let c3 = bfv.homomorphic_addition(c1.clone(), c2.clone());
    let p3 = bfv.decrypt(vec![c3.0.clone(), c3.1.clone()]);
    assert_eq!(p3, (plain1.clone()+plain2.clone()));

    let c4 = bfv.homomorphic_multiplication(c1.clone(), c2.clone());
    let p4 = bfv.decrypt(vec![c4.0.clone(), c4.1.clone()]);
    assert_eq!(p4, plain1.clone()*plain2.clone());

    let c5 = bfv.naive_homomorphic_multiplication(c1, c2);
    let p5 = bfv.decrypt(vec![c5.0.clone(), c5.1.clone(), c5.2.clone()]);
    assert_eq!(p5, plain1*plain2);

}

fn test_rsa() {
    println!("RSA test");

    let rsa: RSA = RSA::init(1024);


    let plain = random_byte_array(12);
    let ciphertext = rsa.encrypt(plain.clone());

    let plaintext = rsa.decrypt(ciphertext.clone());
    assert_eq!(plain, plaintext);
    
}


fn test_kyber() {
    println!("Kyber test");

    let cipher: Kyber512 = Kyber512::init();
    let plain = random_byte_array(32);
   
    let (u, v) = cipher.encrypt(plain.clone());
 
    assert_eq!(cipher.decrypt(u, v), plain);
  
}

fn simple_aes_test() {
    println!("Simple AES");

    /*
   
        Simple AES TEST
    */
    let plaintext: Vec<u8> = random_byte_array(16);
    let mut cipher = aes_factory::init(Modes::GCM, Paddings::PKCS7, AES_KEY_SIZE::AES_256);

    let ciphertext = cipher.encrypt(plaintext.clone());

    assert_eq!(plaintext, cipher.decrypt(ciphertext));


}

fn aes_ctr_preprocessing_test() {
    println!("AES CTR test");

    /*
   
        CTR AES TEST
    */

    let plaintext: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15, 16,17,18,20,21,21,22,33];
    let mut cipher = aes_factory::init(Modes::CTR, Paddings::PKCS7, AES_KEY_SIZE::AES_256);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
}


fn aes_cbc_test() {
    println!("AES CBC test");

    let plaintext: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15, 16,17,18,20,21,21,22,33];
    let mut cipher = aes_factory::init(Modes::CBC, Paddings::PKCS7, AES_KEY_SIZE::AES_256);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
}


fn aes_ecb_test() {
    println!("AES ECB test");

    let plaintext: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15, 16,17,18,20,21,21,22,33];
    let mut cipher = aes_factory::init(Modes::ECB, Paddings::PKCS7, AES_KEY_SIZE::AES_256);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
}