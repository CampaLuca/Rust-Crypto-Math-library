use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use crate::{matrices::{matrix::Matrix, vector::Vector}, algebras::{Rings::{instances::PolynomialRing_instance::PolynomialRingInstance, classes::PolynomialRing::PolynomialRing}, FiniteField::{instances::Zmod_instance::ZmodInstance, classes::Zmod::Zmod}}, cryptography::asymmetric::interfaces::interfaces::{PKIinterface, KEMinterface, LatticeBased_PKIinterface}, poly::univariate_polynomial::UnivariatePolynomial, variables::vars::Var, numbers::{numbers::{Instance, Operand, Number, Class}, instances::{ZZ_instance::{ZZinstance, self}, RR_instance::RRinstance}, classes::RR::RR}, arith::random::{gen_from_uniform_distribution_with_modulo, random_byte_array}};
use crate::arith::random::gen_from_centered_binomial_distribution;
use crate::numbers::classes::ZZ::ZZ;


// kyber utilities
fn compress<T>(poly: UnivariatePolynomial<T>, modulo: BigInt, d: usize) -> UnivariatePolynomial<ZmodInstance> where T: Instance + Clone + Eq + Operand + Number {
    let q1: BigInt = BigInt::from(2).pow(d as u32);
    let r_class: RR = RR::new();
    let factor: RRinstance = r_class.new_instance(BigDecimal::from(q1.clone())/BigDecimal::from(modulo));
    let mut new_poly = (r_class.apply_to_univariate_poly(poly) * factor).round() % q1; // values are yet modulo q
    new_poly
}


fn decompress<T>(poly: UnivariatePolynomial<T>, modulo: BigInt, d: usize) -> UnivariatePolynomial<ZmodInstance> where T: Instance + Clone + Eq + Operand + Number {
    let q1: BigInt = BigInt::from(2).pow(d as u32);
    let r_class: RR = RR::new();
    let factor: RRinstance = r_class.new_instance(BigDecimal::from(modulo.clone())/BigDecimal::from(q1));
    let mut new_poly = (r_class.apply_to_univariate_poly(poly) * factor).round() % modulo; // values are yet modulo q
    new_poly
}

fn plaintext_to_poly(plaintext: Vec<u8>) -> UnivariatePolynomial<ZZinstance> {
    let class = ZZ::new();
    let mut coefficients: Vec<ZZinstance> = Vec::new();
    for el in plaintext {
        for bit in 0..8 {
            coefficients.push(class.new_instance(BigInt::from(el & 1<<bit)>>bit));
        }    
    }

    UnivariatePolynomial::new(coefficients, Var::new("x", BigInt::one()), None)
}


fn poly_to_plaintext(poly: UnivariatePolynomial<ZmodInstance>) -> Vec<u8> {
    let number_of_bytes = (poly.degree()+1)/8;
    let mut plaintext: Vec<u8> = Vec::new();

    for i in 0..number_of_bytes {
        let mut accumulator: u8 = 0;
        for bit in 0..8 {
            accumulator = (poly.coefficients[i*8+bit].value.to_u8().unwrap() & (1<<bit)) >>bit;
        }
        plaintext.push(accumulator);
    }
    plaintext
}

pub struct Kyber512 {
    n: usize,
    public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)>, // n, e
    private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>>,// p, q, d
    primary_key: usize,
    ring: PolynomialRing<ZmodInstance>,
    field: Zmod,
    k: usize,
    eta1: usize,
    eta2: usize,
    du: usize,
    dv: usize
}

pub struct Kyber768 {
    n: usize,
    public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)>, // n, e
    private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>>,// p, q, d
    primary_key: usize,
    ring: PolynomialRing<ZmodInstance>,
    field: Zmod,
    k: usize,
    eta1: usize,
    eta2: usize,
    du: usize,
    dv: usize
}

pub struct Kyber1024 {
    n: usize,
    public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)>, // n, e
    private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>>,// p, q, d
    primary_key: usize,
    ring: PolynomialRing<ZmodInstance>,
    field: Zmod,
    k: usize,
    eta1: usize,
    eta2: usize,
    du: usize,
    dv: usize
}


impl Kyber512 {
    pub fn init() -> Kyber512 {
        let n: usize = 256;
        let q: BigInt = BigInt::from(3329);
        let k: usize = 2; // number of polynomials per vector
        let eta_1: usize = 3;
        let eta_2: usize = 2;
        let du: usize = 10;
        let dv: usize = 4;

        // initializing the struct
        let mut public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)> = Vec::new(); // n, e
        let mut private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        let primary_key: usize = 0;


        // creating the RING x^256+1
        let field: Zmod = Zmod::new(Some(q.clone()));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one());
        for _i in 1..n {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());

        let irreducible_polynomial = UnivariatePolynomial::new(coefficients, var, None);
        let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

        // creating public key, private key couple
        let mut secret_key: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            secret_key.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let s: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(secret_key);

        let mut vectors_of_public_key: Vec<Vec<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        for _i in 0..k {
            let mut vect: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
            for _j in 0..k {
                vect.push(gen_from_uniform_distribution_with_modulo::<BigInt>(BigInt::zero(), BigInt::from(q.clone()), n-1, q.clone()).quotient(irreducible_polynomial.clone()))
            }
        } 
        let A: Matrix<PolynomialRingInstance<ZmodInstance>> = Matrix::new(vectors_of_public_key, k, k);
        
        let mut error: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            error.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let e: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error);

        let b = A.clone()*s.clone() + e;

        public_keys.push((A, b));
        private_keys.push(s);

        Kyber512 { n: n, public_keys: public_keys, private_keys: private_keys, primary_key: primary_key, ring: ring, field: field, k: k, eta1: eta_1, eta2: eta_2, du: du, dv: dv }
        
    }

    //fn keygen() {}
}


impl Kyber768 {
    pub fn init() -> Kyber768 {
        let n: usize = 256;
        let q: BigInt = BigInt::from(3329);
        let k: usize = 3; // number of polynomials per vector
        let eta_1: usize = 2;
        let eta_2: usize = 2;
        let du: usize = 10;
        let dv: usize = 4;

        // initializing the struct
        let mut public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)> = Vec::new(); // n, e
        let mut private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        let primary_key: usize = 0;


        // creating the RING x^256+1
        let field: Zmod = Zmod::new(Some(q.clone()));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one());
        for _i in 1..n {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());

        let irreducible_polynomial = UnivariatePolynomial::new(coefficients, var, None);
        let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

        // creating public key, private key couple
        let mut secret_key: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            secret_key.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let s: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(secret_key);

        let mut vectors_of_public_key: Vec<Vec<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        for _i in 0..k {
            let mut vect: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
            for _j in 0..k {
                vect.push(gen_from_uniform_distribution_with_modulo::<BigInt>(BigInt::zero(), BigInt::from(q.clone()), n-1, q.clone()).quotient(irreducible_polynomial.clone()))
            }
        } 
        let A: Matrix<PolynomialRingInstance<ZmodInstance>> = Matrix::new(vectors_of_public_key, k, k);
        
        let mut error: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            error.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let e: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error);

        let b = A.clone()*s.clone() + e;

        public_keys.push((A, b));
        private_keys.push(s);

        Kyber768 { n: n, public_keys: public_keys, private_keys: private_keys, primary_key: primary_key, ring: ring, field: field, k: k, eta1: eta_1, eta2: eta_2, du: du, dv: dv }
        
    }

    //fn keygen() {}
}


impl Kyber1024 {
    pub fn init() -> Kyber1024 {
        let n: usize = 256;
        let q: BigInt = BigInt::from(3329);
        let k: usize = 4; // number of polynomials per vector
        let eta_1: usize = 2;
        let eta_2: usize = 2;
        let du: usize = 11;
        let dv: usize = 5;

        // initializing the struct
        let mut public_keys: Vec<(Matrix<PolynomialRingInstance<ZmodInstance>>, Vector<PolynomialRingInstance<ZmodInstance>>)> = Vec::new(); // n, e
        let mut private_keys: Vec<Vector<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        let primary_key: usize = 0;


        // creating the RING x^256+1
        let field: Zmod = Zmod::new(Some(q.clone()));
        let var: Var = Var::new("x", BigInt::from(1));
        let mut coefficients: Vec<ZmodInstance> = Vec::new();
        coefficients.push(field.one());
        for _i in 1..n {
            coefficients.push(field.zero());
        }
        coefficients.push(field.one());

        let irreducible_polynomial = UnivariatePolynomial::new(coefficients, var, None);
        let ring: PolynomialRing<ZmodInstance> = PolynomialRing::new(irreducible_polynomial.clone());

        // creating public key, private key couple
        let mut secret_key: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            secret_key.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let s: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(secret_key);

        let mut vectors_of_public_key: Vec<Vec<PolynomialRingInstance<ZmodInstance>>> = Vec::new();
        for _i in 0..k {
            let mut vect: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
            for _j in 0..k {
                vect.push(gen_from_uniform_distribution_with_modulo::<BigInt>(BigInt::zero(), BigInt::from(q.clone()), n-1, q.clone()).quotient(irreducible_polynomial.clone()))
            }
        } 
        let A: Matrix<PolynomialRingInstance<ZmodInstance>> = Matrix::new(vectors_of_public_key, k, k);
        
        let mut error: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..k {
            error.push((gen_from_centered_binomial_distribution(n, eta_1) % q.clone()).quotient(irreducible_polynomial.clone()));
        }
        let e: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error);

        let b = A.clone()*s.clone() + e;

        public_keys.push((A, b));
        private_keys.push(s);

        Kyber1024 { n: n, public_keys: public_keys, private_keys: private_keys, primary_key: primary_key, ring: ring, field: field, k: k, eta1: eta_1, eta2: eta_2, du: du, dv: dv }
        
    }

    //fn keygen() {}
}

impl LatticeBased_PKIinterface for Kyber512 {
    fn encrypt(&self, plaintext: Vec<u8>) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if plaintext.len() > 8 {
            panic!("Plaintext to big. Only 8 bytes can be encrypted");
        }

        let poly_plaintext: UnivariatePolynomial<ZZinstance> = plaintext_to_poly(plaintext);
        let m_scaled: UnivariatePolynomial<ZmodInstance> = decompress(poly_plaintext, self.field.module.as_ref().unwrap().clone(), 1);
        let vector_m_scaled: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(vec![m_scaled.quotient(self.ring.irreducible_polynomial.clone())]);
        let mut random_vector: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            random_vector.push((gen_from_centered_binomial_distribution(self.n, self.eta1) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let r: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(random_vector);

        let mut error_1: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            error_1.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let e1: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_1);

        let mut error_2: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        error_2.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone())); 
        let e2: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_2);

        let (mut A, mut b) = self.public_keys[self.primary_key].clone();

        A.inplace_transpose();
        let mut u =  A*r.clone() + e1;
        let mut v =  (b.transpose()*r + e2 + vector_m_scaled).values[0].clone();

        for i in 0..u.len {
            u.values[i] = compress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone());
        }

        v = compress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());

        (u,v)
    }

    fn decrypt(&self, u: Vector<PolynomialRingInstance<ZmodInstance>>, v: PolynomialRingInstance<ZmodInstance>) -> Vec<u8> {
        let mut decomp_u: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for i in 0..u.len {
            decomp_u.push(decompress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let U: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(decomp_u);    
        let mut decomp_v = decompress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());
        
        let mut temp_result = v - (self.private_keys[self.primary_key].transpose()*U).values[0].clone();
        let compress_result = compress(temp_result.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), 1);

        poly_to_plaintext(compress_result)
    
    }
}


impl KEMinterface for Kyber512 {
    fn encapsulate(&self, bytes_length: usize) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if bytes_length > 8 {
            panic!("The session key could be at maximum 8 bytes long");
        }

        let ss: Vec<u8> = random_byte_array(bytes_length);
        self.encrypt(ss)
    }
}


impl LatticeBased_PKIinterface for Kyber768 {
    fn encrypt(&self, plaintext: Vec<u8>) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if plaintext.len() > 8 {
            panic!("Plaintext to big. Only 8 bytes can be encrypted");
        }

        let poly_plaintext: UnivariatePolynomial<ZZinstance> = plaintext_to_poly(plaintext);
        let m_scaled: UnivariatePolynomial<ZmodInstance> = decompress(poly_plaintext, self.field.module.as_ref().unwrap().clone(), 1);
        let vector_m_scaled: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(vec![m_scaled.quotient(self.ring.irreducible_polynomial.clone())]);
        let mut random_vector: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            random_vector.push((gen_from_centered_binomial_distribution(self.n, self.eta1) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let r: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(random_vector);

        let mut error_1: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            error_1.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let e1: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_1);

        let mut error_2: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        error_2.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone())); 
        let e2: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_2);

        let (mut A, mut b) = self.public_keys[self.primary_key].clone();

        A.inplace_transpose();
        let mut u =  A*r.clone() + e1;
        let mut v =  (b.transpose()*r + e2 + vector_m_scaled).values[0].clone();

        for i in 0..u.len {
            u.values[i] = compress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone());
        }

        v = compress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());

        (u,v)
    }

    fn decrypt(&self, u: Vector<PolynomialRingInstance<ZmodInstance>>, v: PolynomialRingInstance<ZmodInstance>) -> Vec<u8> {
        let mut decomp_u: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for i in 0..u.len {
            decomp_u.push(decompress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let U: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(decomp_u);    
        let mut decomp_v = decompress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());
        
        let mut temp_result = v - (self.private_keys[self.primary_key].transpose()*U).values[0].clone();
        let compress_result = compress(temp_result.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), 1);

        poly_to_plaintext(compress_result)
    
    }
}


impl KEMinterface for Kyber768 {
    fn encapsulate(&self, bytes_length: usize) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if bytes_length > 8 {
            panic!("The session key could be at maximum 8 bytes long");
        }

        let ss: Vec<u8> = random_byte_array(bytes_length);
        self.encrypt(ss)
    }
}

impl LatticeBased_PKIinterface for Kyber1024 {
    fn encrypt(&self, plaintext: Vec<u8>) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if plaintext.len() > 8 {
            panic!("Plaintext to big. Only 8 bytes can be encrypted");
        }

        let poly_plaintext: UnivariatePolynomial<ZZinstance> = plaintext_to_poly(plaintext);
        let m_scaled: UnivariatePolynomial<ZmodInstance> = decompress(poly_plaintext, self.field.module.as_ref().unwrap().clone(), 1);
        let vector_m_scaled: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(vec![m_scaled.quotient(self.ring.irreducible_polynomial.clone())]);
        let mut random_vector: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            random_vector.push((gen_from_centered_binomial_distribution(self.n, self.eta1) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let r: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(random_vector);

        let mut error_1: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for _i in 0..self.k {
            error_1.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let e1: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_1);

        let mut error_2: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        error_2.push((gen_from_centered_binomial_distribution(self.n, self.eta2) % self.field.module.as_ref().unwrap().clone()).quotient(self.ring.irreducible_polynomial.clone())); 
        let e2: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(error_2);

        let (mut A, mut b) = self.public_keys[self.primary_key].clone();

        A.inplace_transpose();
        let mut u =  A*r.clone() + e1;
        let mut v =  (b.transpose()*r + e2 + vector_m_scaled).values[0].clone();

        for i in 0..u.len {
            u.values[i] = compress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone());
        }

        v = compress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());

        (u,v)
    }

    fn decrypt(&self, u: Vector<PolynomialRingInstance<ZmodInstance>>, v: PolynomialRingInstance<ZmodInstance>) -> Vec<u8> {
        let mut decomp_u: Vec<PolynomialRingInstance<ZmodInstance>> = Vec::new();
        for i in 0..u.len {
            decomp_u.push(decompress(u.values[i].unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.du).quotient(self.ring.irreducible_polynomial.clone()));
        }
        let U: Vector<PolynomialRingInstance<ZmodInstance>> = Vector::new(decomp_u);    
        let mut decomp_v = decompress(v.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), self.dv).quotient(self.ring.irreducible_polynomial.clone());
        
        let mut temp_result = v - (self.private_keys[self.primary_key].transpose()*U).values[0].clone();
        let compress_result = compress(temp_result.unwrap_from_ring(), self.field.module.as_ref().unwrap().clone(), 1);

        poly_to_plaintext(compress_result)
    
    }
}


impl KEMinterface for Kyber1024 {
    fn encapsulate(&self, bytes_length: usize) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>) {
        if bytes_length > 8 {
            panic!("The session key could be at maximum 8 bytes long");
        }

        let ss: Vec<u8> = random_byte_array(bytes_length);
        self.encrypt(ss)
    }
}