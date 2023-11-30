use sage_math::test::test_ZZ;
use sage_math::test::test_RR;
use sage_math::test::test_QQ;
use sage_math::variables::vars::Var;
use num_bigint::BigInt;
use sage_math::numbers::instances::ZZ_instance::ZZinstance;
use sage_math::poly::monomial::Monomial;
use num_traits::Pow;


fn main() {
    test_ZZ::test();
    test_RR::test();
    test_QQ::test();

    let v: Var = Var::new("x", BigInt::from(3));
    let w: Var = Var::new("x", BigInt::from(4));
    let m: Monomial<ZZinstance> = Monomial::<ZZinstance>::new_from_var(v.clone());
    let n: Monomial<ZZinstance> = Monomial::<ZZinstance>::new_from_var(w.clone());
    println!("{0}",Pow::pow(w.clone(), BigInt::from(-3))*v);
    
}