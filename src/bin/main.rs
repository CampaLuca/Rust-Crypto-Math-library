use sage_math::algebras::FiniteField::classes::Zmod::Zmod;
use sage_math::algebras::FiniteField::instances::Zmod_instance::ZmodInstance;
use sage_math::algebras::Rings::classes::PolynomialRing::PolynomialRing;
use sage_math::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use sage_math::arith::random;
use sage_math::cryptography::asymmetric::interfaces::interfaces::PKIinterface;
use sage_math::cryptography::asymmetric::primitives::kyber::Kyber512;
use sage_math::cryptography::asymmetric::primitives::rsa::RSA;
use sage_math::numbers::classes::ZZ::ZZ;
use sage_math::numbers::numbers::Operand;
use sage_math::poly::classes::monomial::Monomial;
use sage_math::poly::classes::univariate_polynomial::UnivariatePolynomial;
use sage_math::test::test_ZZ;
use sage_math::test::test_RR;
use sage_math::test::test_QQ;
use sage_math::variables::vars::Var;
use num_bigint::BigInt;
use sage_math::numbers::instances::ZZ_instance::ZZinstance;
use sage_math::poly::instances::monomial_instance::MonomialInstance;
use num_traits::Pow;


fn main() {
    test_ZZ::test();
    test_RR::test();
    test_QQ::test();

    let v: Var = Var::new("x", BigInt::from(3));
    let w: Var = Var::new("x", BigInt::from(4));
    let m: MonomialInstance<ZZinstance> = Monomial::new_from_var(v.clone());
    let n: MonomialInstance<ZZinstance> = Monomial::new_from_var(w.clone());
    println!("{0}",Pow::pow(w.clone(), BigInt::from(-3))*v);

    
    let n: usize = 8;
        let q: ZZinstance = ZZ::new().new_instance(BigInt::from(23));
    let field: Zmod = Zmod::new(Some(q.clone()));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one());
        for _i in 1..n {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());
    //let cipher: Kyber512 = Kyber512::init();
    let irreducible_polynomial = UnivariatePolynomial::new_instance(coefficients, var, None);
    let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

    let a: PolynomialRingInstance<ZmodInstance> = (random::gen_from_centered_binomial_distribution(n, 2) % q.clone()).quotient(irreducible_polynomial.clone());
    let b: PolynomialRingInstance<ZmodInstance> = (random::gen_from_centered_binomial_distribution(n, 2) % q.clone()).quotient(irreducible_polynomial.clone());

    println!("{}",a);
    println!("{}",b);
    
    let c = a*b;
    println!("{}",c);

    let rsa: RSA = RSA::init(120);
    println!("{}", rsa.private_keys[0].0);
    println!("{}", rsa.private_keys[0].1);
    println!("{}", rsa.private_keys[0].2);
    println!("{}", rsa.public_keys[0].1);

    println!("{}", (BigInt::from(-2).modpow(&BigInt::from(1), &BigInt::from(5))));

    let plain = vec![1,2,3,4,5,6,7];
    let ciphertext = rsa.encrypt(plain);

    let plaintext = rsa.decrypt(ciphertext.clone());
    println!("{:?}",ciphertext);
    println!("{:?}",plaintext);
    
   
}