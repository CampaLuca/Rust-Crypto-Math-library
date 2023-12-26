use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use bigdecimal::RoundingMode;
use num_integer::Integer;
use num_integer::ExtendedGcd;
use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Number;
use crate::numbers::numbers::Operand;
use crate::algebras::Rings::instances::PolynomialRing_instance::PolynomialRingInstance;
use crate::poly::univariate_polynomial::UnivariatePolynomial;


pub fn round_to_bigint(a: BigDecimal) -> BigInt {
    let (bigint, _decimal_part_digits) = a.round(0).into_bigint_and_exponent();
    bigint
}

pub fn truncate(a: BigDecimal) -> BigInt {
    let (bigint, _decimal_part_digits) = a.with_scale_round(0, RoundingMode::Floor).into_bigint_and_exponent();
    bigint
}

pub fn modular_inverse(a: BigInt, module: BigInt) -> BigInt {
    let (ExtendedGcd {gcd: _, x, y: _, ..}, _lcm) = a.extended_gcd_lcm(&module);
    x
}

pub fn poly_divmod<T>(p: &UnivariatePolynomial<T>, q: &UnivariatePolynomial<T>) -> Vec<UnivariatePolynomial<T>> where T: Instance + Clone + PartialEq + Operand + Number {
    if (*q).clone() == UnivariatePolynomial::zero(q.var.clone()) {
        panic!("Cannot divide by 0");
    } else {
        let mut l: UnivariatePolynomial<T> = UnivariatePolynomial::zero(q.var.clone());
        let mut r: UnivariatePolynomial<T> = (*p).clone();
        let var = q.var.clone();
        while r != UnivariatePolynomial::zero(p.var.clone()) && q.clone().degree() <= r.clone().degree() {
            let t = r.clone().leading_coefficient().div(&(q.leading_coefficient()));
            let mut coeff_m: Vec<T> = Vec::new();
            for _i in 0..(r.clone().degree()-q.clone().degree()) {
                coeff_m.push(T::zero());
            }
            coeff_m.push(T::one());
            let m: UnivariatePolynomial<T> = UnivariatePolynomial::new(coeff_m, var.clone(), None);
            l = l+m.clone()*t.clone();
            r = r.clone()-((*q).clone()*m*t);

        }

        let mut result: Vec<UnivariatePolynomial<T>> = Vec::new();
        result.push(l);
        result.push(r);
        result
        
    }
}



// def div(p,q):
//     if q==0:
//         return("NaN")
//     elif q!=0:
//         l=0
//         r=p
//         while r!=0 and q.degree()<=r.degree():
//             t=r.leading_coefficient()/q.leading_coefficient()
//             m=x^r.degree()/x^q.degree()
//             m=R(m)
//             l=l+t*m
//             r=r-(t*m*q)
//             print(l,r)
//         return(l,r)

pub fn egcd<T>(a: PolynomialRingInstance<T>, b: PolynomialRingInstance<T>) -> Vec<PolynomialRingInstance<T>> where T: Instance + Clone + PartialEq + Operand + Number {
    /*
    Extended Euclidean Algorithm (iterative)
    Return (d, x, y) where:
        - d is the GCD of a and b
        - x, y are integers satisfying: a*x + b*y = d
        - b != 0
        - PostConditions: abs(x) <= abs(b//d) and abs(y) <= abs(a//d)
    */
    let var = a.var.clone();
    let mut a_tuple= vec![a.clone(), a.class.clone().into_inner().one(var.clone()), a.class.clone().into_inner().zero(var.clone())];
    let mut b_tuple = vec![b.clone(), a.class.clone().into_inner().zero(var.clone()), a.class.clone().into_inner().one(var.clone())];
    loop {
        let qr: Vec<UnivariatePolynomial<T>> = poly_divmod(&(a_tuple[0].unwrap_from_ring()), &(b_tuple[0].unwrap_from_ring()));
        let q = a.class.clone().into_inner().apply(&qr[0]);
        let r = a.class.clone().into_inner().apply(&qr[1]);
        if r == a.class.clone().into_inner().zero(var.clone()) {
            return b_tuple;
        }
        a_tuple = b_tuple.clone();
        b_tuple = vec![r, a_tuple[1].clone()-q.clone()*b_tuple[1].clone(), a_tuple[2].clone()-q.clone()*b_tuple[2].clone()];
    }
}

// def egcd(a: int, b: int) -> Tuple[int, int, int]:
//     """ Extended Euclidean algorithm (iterative).
//         Returns (d, x, y) where d is the Greatest Common Divisor of a and b.
//         x, y are integers that satisfy: a*x + b*y = d
//         Precondition: b != 0
//         Postcondition: abs(x) <= abs(b//d) and abs(y) <= abs(a//d) """
//     a = (a, 1, 0)
//     b = (b, 0, 1)
//     while True:
//         q, r = divmod(a[0], b[0])
//         if not r: return b
//         a, b = b, (r, a[1] - q*b[1], a[2] - q*b[2])