use std::fmt::Display;

use num_bigint::BigInt;
use num_prime::BitTest;
use num_traits::{pow, Pow, Signed, ToPrimitive};
use crate::{algebras::FiniteField::{classes::Zmod::Zmod, instances::Zmod_instance::ZmodInstance}, arith::{primes::{self, is_prime}, random::{get_random_bigint, get_random_bigint_with_bounds}}, numbers::{classes::ZZ::ZZ, numbers::{Class, Instance, Operand}}};

#[derive(Clone)]
pub enum NTT_Algorithm {
    Naive,
    CooleyTukey,
    Iterative,
    NegativeConvolution
}

#[derive(Clone)]
pub struct NTT {
    pub DOUBLEROU: bool,
    pub DEBUG: bool,
    pub N: usize,
    pub q: BigInt,
    pub zeta: BigInt,
    pub zetas: Vec<BigInt>,
    pub zetas_inv: Vec<BigInt>,
    pub post_proc: BigInt,
    pub prec: i32,
    pub ntt_inv: BigInt,
    pub findeg: usize,
    pub nth_root_of_unity: BigInt,
    pub inv_nth_root_of_unity: BigInt,
    pub ntt_algorithm: NTT_Algorithm
}


impl NTT {
    pub fn new(DOUBLEROU: bool, DEBUG: bool,
        N: usize,
        q: BigInt,
        zeta: BigInt,
        zetas: Vec<BigInt>,
        zetas_inv: Vec<BigInt>,
        post_proc: BigInt,
        prec: i32,
        ntt_inv: BigInt,
        findeg: usize,
        nth_root_of_unity: BigInt,
        inv_nth_root_of_unity: BigInt,
        ntt_algorithm: NTT_Algorithm) -> NTT {
        NTT { DOUBLEROU: DOUBLEROU, DEBUG: DEBUG, N: N, q: q, zeta: zeta, zetas: zetas, zetas_inv: zetas_inv, post_proc: post_proc, prec: prec, ntt_inv: ntt_inv, findeg: findeg, nth_root_of_unity, inv_nth_root_of_unity, ntt_algorithm}
    }
}

// methods for generating NTT primes
impl NTT {

    pub fn generate_ntt_prime(N: usize, bit_length: usize, DOUBLEROU: bool) -> (Option<BigInt>, Option<BigInt>) {
        let mut p: BigInt = BigInt::from(1);
        loop {
            p = get_random_bigint(bit_length as u64).abs();
            if DOUBLEROU {
                if is_prime(p.to_biguint().unwrap()) && (p.clone()-1)%(N.clone()*4) == BigInt::from(0) {
                    break;
                }
            } else {
                if is_prime(p.to_biguint().unwrap()) && (p.clone()-1)%(N.clone()*2) == BigInt::from(0) && (p.clone()-1)%(N.clone()*4) != BigInt::from(0) {
                    break;
                }
            }
        }

        if DOUBLEROU {
            for k in (1..bit_length).rev() {
                if (p.clone()-1)%BigInt::from(2).pow(k) == BigInt::from(0) {
                    return (Some(p.clone()), Some((p-1)/BigInt::from(2).pow(k)))
                }
            }

            return (None, None)
        }  else {
            return (Some(p.clone()), Some((p-1)/N))
        }
    }

}

fn get_factors(v:BigInt) -> Vec<BigInt> {
    let factors_ =  num_prime::nt_funcs::factorize(v.to_u128().unwrap());
    let mut factors : Vec<BigInt> = Vec::new();

    for (p, e) in factors_.into_iter() {
        factors.push(BigInt::from(p));
    }

    factors

}

// function for primitive root searching

impl NTT {
    pub fn get_primitive_root_kullanilabilir( q: BigInt, m: u64) -> BigInt{
        loop {
            let zeta: BigInt = get_random_bigint_with_bounds(BigInt::from(0), q.clone()-1);
            let mut check: bool = true;

            for i in 1..m {
                if zeta.modpow(&BigInt::from(i), &q) == BigInt::from(1)  {
                    check = false;
                    break;
                }
            }

            if zeta.modpow(&BigInt::from(m), &q) == BigInt::from(1) && check {
                return zeta;
            }
        }

        return BigInt::from(0);
    }


    pub fn get_primitive_root_simple( q: BigInt, m: u64) -> BigInt {
        loop {
            let zeta: BigInt = get_random_bigint_with_bounds(BigInt::from(0), q.clone()-1);

            if zeta.modpow(&BigInt::from(m), &q) == BigInt::from(1)  {
                if zeta.modpow(&BigInt::from(m/2), &q) == q.clone()-1 {
                    return zeta;
                }
            }
        }

    }

    pub fn get_primitive_root_through_generator(q: BigInt, k: BigInt, DOUBLEROU: bool) -> Option<BigInt>{
        // if DOUBLEROU is True k = 2^(k')
        // else k = integer
        if DOUBLEROU {
            let factor: BigInt = BigInt::from(2);
            let mut generator: BigInt = BigInt::from(1);
            for g in num_iter::range_inclusive(BigInt::from(1), q.clone()-1) {
                let check: bool = g.modpow(&((q.clone()-1)/2), &q.clone()) != BigInt::from(1);
                if check {
                    generator = g;
                    break;
                }
            }
            if generator == BigInt::from(1) {
                return None;
            }

            return Some(generator.modpow(&((q.clone()-1)/k), &q));
        }else {
            let mut generator: BigInt = BigInt::from(1);
            let factors: Vec<BigInt> = get_factors(q.clone()-1);
            for g in num_iter::range_inclusive(BigInt::from(1), q.clone()-1) {
                let mut check: bool = true;
                for factor in factors.clone() {
                    if g.modpow(&((q.clone()-1)/factor), &q) == BigInt::from(1) {
                        check = false;
                        break;
                    }
                }

                if check {
                    generator = g;
                    break;
                }
            }

            if generator == BigInt::from(1) {
                return None;
            }

            return Some(generator.modpow(&k, &q))
        }
    }

    pub fn get_primitive_root_directly(q: BigInt, k: BigInt) -> Option<BigInt> {
        let n: BigInt = (q.clone()-1)/k;
        let factors: Vec<BigInt> = get_factors(n.clone());

        for a in num_iter::range_inclusive(BigInt::from(1), q.clone()-1) {
            if a.modpow(&n, &q) == BigInt::from(1) {
                let mut check: bool = true;
                for factor in factors.clone() {
                    if a.modpow(&(n.clone()/factor), &q) == BigInt::from(1) {
                        check = false;
                        break;
                    }
                }

                if check {
                    return Some(a);
                }
            }
        }


        return None

    }


}


// NTT auxiliary functions
impl NTT {
    pub fn gen_powers(N: usize, q: BigInt, zeta: BigInt, findeg: usize) -> Vec<BigInt> {
        if findeg == 1 || findeg == 2 {
            let mut N_ : usize;
            if findeg == 1 {
                N_ = N;
            } else {
                N_ = N/2;
            }

            let mut powers: Vec<BigInt> = vec![BigInt::from(0); N_];
            powers[0] = BigInt::from(0);
            powers[1] = BigInt::from(N_/2);
            let mut index: usize = 1;
            while 2.pow(index) < N_ as i32 {
                for j in num_iter::range_step_inclusive(2.pow(index) as i32, 2.pow(index+1)-1, 2) {
                    powers[j as usize] = powers[(j/2) as usize].clone()/2;
                    powers[(j+1) as usize] = (powers[(j/2) as usize].clone()+N_)/2;    
                }

                index = index + 1;
            }

            return powers;
        } else { // findeg == 3
            let mut N_: usize = N/2;
            let mut powers: Vec<BigInt> = vec![BigInt::from(0); N/3];
            powers[0] = BigInt::from(0);
            powers[1] = BigInt::from(N/6);
            powers[2] = powers[1].clone()/2;
            powers[3] = (5*powers[1].clone())/2;
            let mut index: usize = 2;

            while 2.pow(index) < (N/3) as i32 {
                for j in num_iter::range_step_inclusive(2.pow(index) as i32, 2.pow(index+1)-2.pow(index-1)-1, 2) {
                    powers[j as usize] = powers[(j/2) as usize].clone()/2;
                    powers[(j+1) as usize] = (powers[(j/2) as usize].clone()+N_)/2;
                }

                for j in num_iter::range_step_inclusive((2.pow(index+1)-2.pow(index-1)) as i32, 2.pow(index+1)-1, 2) {
                    powers[j as usize] = powers[(j/2) as usize].clone()/2;
                    powers[(j+1) as usize] = (powers[(j/2) as usize].clone()+N_)/2;
                }

                index = index +1;
            }

            return powers;
        }
    }



    pub fn gen_twiddles(N: usize, q: BigInt, zeta: BigInt, powers: Vec<BigInt>) -> (Vec<BigInt>, Vec<BigInt>) {
        let twiddle_count = powers.len();
        let mut twiddles: Vec<BigInt> = vec![BigInt::from(0); twiddle_count];
        let mut inv_twiddles: Vec<BigInt> = vec![BigInt::from(0); twiddle_count];

        let mut tmp: Vec<BigInt> = vec![BigInt::from(0); N];

        for i in 0..N {
            tmp[i] = zeta.modpow(&BigInt::from(i), &q);
        }

        for i in 0..twiddle_count {
            twiddles[i] = tmp[powers[i].to_usize().unwrap()].clone();
            inv_twiddles[i] = (-(tmp[powers[twiddle_count-1-i].to_usize().unwrap()].clone())).modpow(&BigInt::from(1), &q);
        }

        return (twiddles, inv_twiddles)
    }


    fn exist_small_n(r: BigInt, q: BigInt, N: usize) -> bool{
        for k in 2..N {
            if r.modpow(&BigInt::from(k), &q) == BigInt::from(1) {
                return true;
            }
        }

        return false;
    }

    pub fn get_nth_root_of_unity(q: BigInt, N: usize) -> BigInt {
        if !is_prime(q.clone().to_biguint().unwrap()) {
            panic!("The module must be a prime of the form 2^k * c + 1");
        }

        if (q.clone()-1) % N.clone() != BigInt::from(0) {
            panic!("N must divide phi(module)");
        }

        let phi_q: BigInt = q.clone()-1;

        loop {
            let alpha = get_random_bigint_with_bounds(BigInt::from(1), q.clone());
            let beta = alpha.modpow(&(phi_q.clone()/N), &q);
            if ! NTT::exist_small_n(beta.clone(), q.clone(), N.clone()) {
                return beta;
            }
        }


    }
}


// function for transforming a polynomial in NTT format or viceversa
// problem if j is supposed to be modified by the loop as done in python, can be easily replaced with an other var
impl NTT {
    pub fn cooley_tukey_iterative_ntt<T>(N: usize, q: BigInt, W: BigInt, coefficients: Vec<T>, prec: i32) -> Vec<ZmodInstance> where T:  Instance + Clone + Operand {
      
            let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
            //let rev_coefficients: Vec<T> = NTT::index_reversed(coefficients.clone(), prec.clone());
            let mut poly: Vec<ZmodInstance> = Vec::new();

            for el in coefficients.clone() {
                poly.push(field.apply(el));
            }

           

            poly = NTT::index_reversed(poly.clone(), prec.clone());

            
           
            let v = (prec);
            for i in 0..v {
                let mut points1: Vec<ZmodInstance> = Vec::new();
                let mut points2: Vec<ZmodInstance> = Vec::new();
                let mut points: Vec<ZmodInstance> = Vec::new();

                for j in 0..(N/2) {
                    let shift_bits = v - 1 -i;
                    let P = (j.clone() >> shift_bits) << shift_bits;
                    let w_P = W.modpow(&BigInt::from(P), &q);
                    let odd = poly[2*j+1].clone()*field.apply(w_P.clone());
                    let even = poly[2*j].clone();
                    points1.push(even.clone() + odd.clone());
                    points2.push(even.clone()-odd.clone());

                    points = Vec::new();
                    for el in points1.clone() {
                        points.push(el);
                    }
                    for el in points2.clone() {
                        points.push(el);
                    }
                }
                
                if i != v {
                    poly = points.clone();
                }
                
            }

            
    
            return poly;
        }


    pub fn boh_ntt<T>(N: usize, q: BigInt, W: BigInt, coefficients: Vec<T>, prec: i32, zetas: Vec<BigInt>) -> Vec<ZmodInstance> where T: Instance + Clone + Operand {
        let mut k = 1;
        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let mut f: Vec<ZmodInstance> = Vec::new();
            for el in coefficients {
                f.push(field.apply(el));
            }
        
        let mut j = 0;
        let mut l = N/2;
        while l >= 2 {
            let mut start = 0;
            while start < N {
                let zeta = field.apply(zetas[k].clone());
                k = k + 1;
                for j in start..start+l {
                    let t = zeta.clone()*f[j+l].clone();
                    f[j+l] = f[j].clone()-t.clone();
                    f[j] = f[j].clone()+t;

                }
                j = start+l-1;
                start = l + j + 1

            }

            l = l>>1;
        }

        return f;
       
       }


    

    pub fn iterative_ntt<T>(N: usize, q: BigInt, W: BigInt, coefficients: Vec<T>, prec: i32) -> Vec<ZmodInstance> where T: Instance + Clone + Operand {
      
        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let mut f: Vec<ZmodInstance> = Vec::new();
            for el in coefficients {
                f.push(field.apply(el));
            }

        let v = (prec) as usize;
        for i in 0..v {
            for j in 0..2.pow(i) {
                for k in 0..2.pow(v-i-1) {
                    let s = j*2.pow(v-i)+k;
                    let t = s + 2.pow(v-i-1);

                    let w = field.apply(W.clone().modpow(&BigInt::from(2.pow(i)*k), &q));

                    let as_tmp = f[s as usize].clone();
                    let at_tmp = f[t as usize].clone();

                    f[s as usize] = as_tmp.clone() + at_tmp.clone();
                    f[t as usize] = (as_tmp-at_tmp)*w;

                }
            }
        }

        return f;
    }


    pub fn to_ntt<T>(&self, coefficients: Vec<T>) -> Vec<ZmodInstance> where T: Instance + Clone + Operand   {
        let N = self.N;
        let q = self.q.clone();
        return match self.ntt_algorithm {
            NTT_Algorithm::CooleyTukey => NTT::cooley_tukey_iterative_ntt(self.N.clone(), self.q.clone(), self.nth_root_of_unity.clone(), coefficients, self.prec.clone()),
            NTT_Algorithm::Iterative => NTT::iterative_ntt(self.N.clone(), self.q.clone(), self.nth_root_of_unity.clone(), coefficients, self.prec.clone()),
            NTT_Algorithm::NegativeConvolution => NTT::boh_ntt(self.N.clone(), self.q.clone(), self.inv_nth_root_of_unity.clone(), coefficients, self.prec.clone(), self.zetas.clone()),
            _ => NTT::iterative_ntt(self.N.clone(), self.q.clone(), self.nth_root_of_unity.clone(), coefficients, self.prec.clone()),


        };
    }


    
    // pub fn to_ntt<T>(&self, coefficients: Vec<T>) -> Vec<ZmodInstance> where T: Instance + Clone + Operand  {
    //     let N = self.N;
    //     let q = self.q.clone();
    //     let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));

    //     let mut f: Vec<ZmodInstance> = Vec::new();
    //     for el in coefficients {
    //         f.push(field.apply(el));
    //     }
        
    //     let mut multi_cnt = 0;
    //     let mut k: usize;
    //     let mut length: usize;
    //     if self.findeg == 3 {
    //         length = N/4;
    //         for i in 0..N/2 {
    //             let omega = field.apply(self.zetas[1].clone());
    //             let t = omega.mul(&f[i+N/2].clone());
    //             f[i+N/2] = f[i].clone()+f[i+N/2].clone()-t.clone();
    //             f[i] = f[i].clone()+t;
    //             multi_cnt += 1;
    //         }
    //         k = 2;
    //     } else {
    //         length = N/2;
    //         k = 1;
    //     }

    //     let mut j = 0;
    //     while length >= self.findeg {
    //         let mut start = 0;
    //         while start < N {
    //             if self.DEBUG {
    //                 let mut line = String::new();
    //                 println!("Push any button to continue:");
    //                 let b1 = std::io::stdin().read_line(&mut line).unwrap();
    //             }

    //             let omega = field.apply(self.zetas[k].clone());

    //             if self.DEBUG {
    //                 println!("Zeta power: {}", k);
    //             }

    //             k += 1;

    //             for j in start..start+length {
    //                 if self.DEBUG {
    //                     println!("i, j: {}, {}", j, j+length);
    //                 }

    //                 let t = omega.mul(&f[j+length]);
    //                 f[j+length] = f[j].clone()-t.clone();
    //                 f[j] = f[j].clone()+t.clone();
    //                 multi_cnt += 1;
    //             }

    //             j = start+length-1;

    //             start = (j+1)+length

    //         }

    //         length = length >> 1;
    //     }

    //     return f;
    // }

    fn int_reverse(num: usize, prec: i32) -> usize{
        // let mut rev = 0;

        // for j in 0..prec {
        //     let k_th_bit = (i & (0x1 << j)) >> j;
        //     if k_th_bit == 1 {
        //         rev = rev | (k_th_bit << ((prec)-(j as i32)-1) as usize);
        //     }
        // }
        // return rev;
        let mut rev = 0;

        for i in 0..prec {
            if (num >> i) & 0x1 == 1 {
                rev = rev | (1<<(prec-1-i));
            }
        }

        rev.to_usize().unwrap()
    }

    pub fn index_reversed<T>(coefficients: Vec<T>, prec: i32) -> Vec<T> where T: Clone{
        let N = coefficients.len();
        let mut f: Vec<T> = Vec::new();
        for el in coefficients.clone() {
            f.push(el);
        }
        for i in 0..N {
            let rev_index = NTT::int_reverse(i, prec);
            if rev_index > i {
                f[rev_index] = coefficients[i].clone();
                f[i] = coefficients[rev_index].clone();
            }
        }

        return f;
    }
    

    pub fn cooley_tukey_iterative_intt(N: usize, q: BigInt, W_inv: BigInt, coefficients: Vec<ZmodInstance>, prec: i32, ntt_inv: BigInt) -> Vec<ZmodInstance> {
        let mut f: Vec<ZmodInstance> = Vec::new();
        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let values = NTT::cooley_tukey_iterative_ntt(N, q, W_inv, coefficients.clone(), prec);
        for el in values.clone() {
            f.push(el*field.apply(ntt_inv.clone()));
        }
        return f;
    }
    pub fn boh_intt(N: usize, q: BigInt, W_inv: BigInt, coefficients: Vec<ZmodInstance>, prec: i32, ntt_inv: BigInt, zetas: Vec<BigInt>) -> Vec<ZmodInstance> {
        let mut l = 2;
        let mut l_upper = N/2;
        let mut k = l_upper-1;
        let mut j = 0;
        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let mut f: Vec<ZmodInstance> = coefficients.clone();

        while l <= N/2 {
            let mut start = 0;
            while start < N {
                let zeta = field.apply(zetas[k].clone());
                k = k -1;
                for j in start..start+l {
                    let t = f[j].clone();
                    f[j] = t.clone()+f[j+l].clone();
                    f[j+l] = f[j+l].clone()-t;
                    f[j+l] = zeta.clone()*f[j+l].clone();
                }

                j = start+l-1;
                start = j+l+1;
            }

            l = l<<1;
        }

        for j in 0..f.len() {
            f[j] = f[j].clone()*field.apply(ntt_inv.clone());
        }


        return f;
    }
    pub fn iterative_intt(N: usize, q: BigInt, W_inv: BigInt, coefficients: Vec<ZmodInstance>, prec: i32, ntt_inv: BigInt) -> Vec<ZmodInstance> {
        let mut f: Vec<ZmodInstance> = Vec::new();
        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let values = NTT::index_reversed(NTT::iterative_ntt(N, q, W_inv, NTT::index_reversed(coefficients.clone(), prec.clone()), prec), prec);
        for el in values.clone() {
            f.push(el*field.apply(ntt_inv.clone()));
        }
        return f;
    }

    pub fn from_ntt(&self, coefficients: Vec<ZmodInstance>) -> Vec<ZmodInstance> {
        return match self.ntt_algorithm {
            NTT_Algorithm::CooleyTukey => NTT::cooley_tukey_iterative_intt(self.N.clone(), self.q.clone(), self.inv_nth_root_of_unity.clone(), coefficients, self.prec.clone(), self.ntt_inv.clone()),
            NTT_Algorithm::Iterative => NTT::iterative_intt(self.N.clone(), self.q.clone(), self.inv_nth_root_of_unity.clone(), coefficients, self.prec.clone(), self.ntt_inv.clone()),   
            NTT_Algorithm::NegativeConvolution => NTT::boh_intt(self.N.clone(), self.q.clone(), self.inv_nth_root_of_unity.clone(), coefficients, self.prec.clone(), self.ntt_inv.clone(), self.zetas.clone()),
            _  => NTT::iterative_intt(self.N.clone(), self.q.clone(), self.inv_nth_root_of_unity.clone(), coefficients, self.prec.clone(), self.ntt_inv.clone())

        };
    }

    // pub fn from_ntt(&self, coefficients: Vec<ZmodInstance>) -> Vec<ZmodInstance> {
    //     let N = self.N;
    //     let q = self.q.clone();
    //     let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));

    //     let mut f: Vec<ZmodInstance> = coefficients.clone();
        
        
    //     let mut multi_cnt = 0;
    //     let mut k: usize = 0;
    //     let mut j: usize = 0;
    //     let mut length: usize = self.findeg;
    //     let mut N_: usize;
    //     if self.findeg == 3{
    //         N_ = N/4;
    //     } else {
    //         N_ = N/2;
    //     }

    //     while length <= N_ {
    //         let mut start: usize = 0;
    //         while start < N {
    //             let omega = field.apply(self.zetas_inv[k].clone());
    //             k += 1;
    //             for j in start..start+length {
    //                 let t = f[j].clone();
    //                 f[j] = t.clone() + f[j+length].clone();
    //                 f[j+length] = t.clone()-f[j+length].clone();
    //                 f[j+length] = omega.clone() * f[j+length].clone();
    //                 multi_cnt += 1;
    //             }
    //             j = start+length-1;
    //             start = (j+1) + length;
    //         }
    //         length = length <<1;
    //     }


    //     if self.findeg == 3 {
    //         for i in 0..N/2 {
    //             let t = (f[i].clone()-f[i+N/2].clone())*field.apply(self.post_proc.clone());
    //             f[i] = f[i].clone() + f[i+N/2].clone() -t.clone();
    //             f[i+N/2] = t*field.apply(BigInt::from(2));
    //         }
    //     }

    //     for i in 0..N {
    //         f[i] = f[i].clone()*field.apply(self.ntt_inv.clone());
    //         multi_cnt += 1;
    //     }


    //     return f;

    

    // }


}


#[derive(Clone)]
pub struct NTTFactory {}
impl NTTFactory {
    pub fn init(N: usize, q: BigInt, zeta: BigInt, ntt_algorithm: NTT_Algorithm) -> NTT {

        let mut N2: usize;
        let findeg: usize;
        match ntt_algorithm {
            NTT_Algorithm::NegativeConvolution => findeg = 2,
            _ => findeg = 1
        }
        if findeg == 1 { N2 = N;}
        else if findeg == 2 { N2 = N/2;}
        else { N2 = N/2;}

        let prec = (N2 as f32).log2().floor().to_i32().unwrap();
        let powers = NTT::gen_powers(N, q.clone(), zeta.clone(), 2);
        let (zetas, zetas_inv) = NTT::gen_twiddles(N, q.clone(), zeta.clone(), powers.clone());


        let field = Zmod::new(Some(ZZ::new().new_instance(q.clone())));
        let post_proc = field.apply(2*zetas[1].clone()-1).inverse().value.value;
        
        let ntt_inv: BigInt;

        if findeg == 3 {
            ntt_inv = field.apply(BigInt::from(N)/3).inverse().value.value;
        } else {
            ntt_inv = field.apply(BigInt::from(N2)).inverse().value.value;
        }

        let nth_root_of_unity = NTT::get_nth_root_of_unity(q.clone(), N.clone());
        let inv_nth_root_of_unity = field.apply(nth_root_of_unity.clone()).inverse().value.value;

        
        let ntt_ctxt = NTT::new(
            false, 
            false,
            N,
            q,
            zeta,
            zetas,
            zetas_inv,
            post_proc,
            prec,
            ntt_inv,
            findeg,
        nth_root_of_unity,
    inv_nth_root_of_unity,
ntt_algorithm);
        
        ntt_ctxt
        
    }
}



