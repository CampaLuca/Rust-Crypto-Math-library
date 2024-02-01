# Rust Math and Cryptography library

## This is an ongoing project. It MUST NOT be used in production.
A Rust implementation of some SageMath functions.
It also includes Cryptographic primitives implemented with the same concepts of Google Tink, making schemes interchangable and keys easy to renew, swap or backup.
The aim is to put withing a single library all the currectly used schemes and some of the recently standardized PQ algorithms. 


## Cryptography
Symmetric ciphers:
- [AES 128 ](#aes)

Modes of operation for AES:
- CTR_with_preprocessing, ECB, CBC

Padding Types:
- ANSI_X9_23
- ISO_10126
- PKCS7
- ISO_IEC_7816_4

Asymmetric ciphers:
- [RSA](#rsa)
- [Kyber512, Kyber768, Kyber1024](#kyber)

Homomorphic Encryption:
- [BFV (Brakerski/Fan-Vercauteren)](#bfv)


## Mathematics
Numeric types:
- [ZZ (Integers)](#integers)
- [QQ (Rational)](#rational-numbers)
- [RR (Real)](#real-numbers)

Groups:
- Finite fields (Zmod)
- Polynomial Rings

Linear algebra:
- matrices
- vectors

Polynomials:
- Univariate polynomials
- Monomials
- Multivariate polynomials
- Variables

Transform:
- NTT

Arithmetic:
- functions on primes
- random generators

## Documentation by examples

### Integers
```rust
fn test_sum() {
    let zz_class = ZZ::new();
    let v1: ZZinstance = zz_class.apply(BigInt::from(3));
    //let v2: ZZ_instance = zz_class.apply(BigInt::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: ZZinstance = zz_class.apply(BigInt::parse_bytes(b"13254435464566445343233245346146452342324398292378293724254895732984734987322498749837432493873249873498327498734983498743", 10).unwrap());
    let v3: ZZinstance = v1 + v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::parse_bytes(b"13254435464566445343233245346146452342324398292378293724254895732984734987322498749837432493873249873498327498734983498746", 10).unwrap());
    println!("{}", value);

}

fn test_sub() {
    let zz_class = ZZ::new();
    let v1: ZZinstance = zz_class.apply(BigInt::from(2));
    let v2: ZZinstance = zz_class.apply(BigInt::from(3));
    let v3: ZZinstance = v1 - v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(-1));
    println!("{}", value);

}

fn test_mul() {
    let zz_class = ZZ::new();

    let v1: ZZinstance = zz_class.apply(BigInt::from(2));
    let v2: ZZinstance = zz_class.apply(BigInt::from(-3));
    let v3: ZZinstance = v1 * v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(-6));
    println!("{}", value);

}

fn test_div() {
    let zz_class = ZZ::new();

    let v1: ZZinstance = zz_class.apply(BigInt::from(3));
    let v2: ZZinstance = zz_class.apply(BigInt::from(2));
    let v3: ZZinstance = v1 / v2;
    let value = v3.value.clone();
    assert_eq!(value, BigInt::from(1));
    println!("{}", value);
}
```
### Rational numbers
```rust
fn test_sum() {
    let qq_class: QQ = QQ::new();
    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 + v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_sub() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 - v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_mul() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 * v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}

fn test_div() {
    let qq_class: QQ = QQ::new();

    let v1: QQinstance = qq_class.new_instance(BigInt::from(3),BigInt::from(4));
    //let v2: QQ_instance = QQ_class.new_instance(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: QQinstance = qq_class.new_instance(BigInt::from(5),BigInt::from(6));
    let v3: QQinstance = v1 / v2;
    
    println!("{0}/{1}", v3.numerator.clone() , v3.denominator.clone());

    let ff = qq_class.apply(BigDecimal::from_str("1.5").unwrap());
    println!("{0}/{1}", ff.numerator, ff.denominator);
   
}
```

### Real numbers
```rust
fn test_sum() {
    let rr_class: RR = RR::new();
    let v1: RRinstance = rr_class.apply(BigDecimal::from_str("123.45").unwrap());
    //let v2: RR_instance = rr_class.apply(BigDecimal::from("1325443546456644534323324534614645234232439829237829372"));
    let v2: RRinstance = rr_class.apply(BigDecimal::from_str("123.46").unwrap());
    let v3: RRinstance = v1 + v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from_str("246.91").unwrap());
}

fn test_sub() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from_str("2.56").unwrap());
    let v2: RRinstance = rr_class.apply(BigDecimal::from_str("343565246334.32").unwrap());
    let v3: RRinstance = v1 - v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from_str("-343565246331.76").unwrap());
}

fn test_mul() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from(2));
    let v2: RRinstance = rr_class.apply(BigDecimal::from(-3));
    let v3: RRinstance = v1 * v2;
    let value = v3.value.clone();
    assert_eq!(value, BigDecimal::from(-6));
}

fn test_div() {
    let rr_class: RR = RR::new();

    let v1: RRinstance = rr_class.apply(BigDecimal::from(3));
    let v2: RRinstance = rr_class.apply(BigDecimal::from(10));
    let v3: RRinstance = v1 / v2;
    let value = v3.value.clone();
    println!("{} is the result of 1/3", value);
    //assert_eq!(value, BigDecimal::from_str("1.5").unwrap());
}
```

### AES
#### Simple AES
```rust
    let plaintext: Vec<u8> = random_byte_array(16);
    let mut cipher = aes_factory::init(Modes::NONE, Paddings::NONE);
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
```

#### AES_ECB
```rust
    let plaintext: Vec<u8> = random_byte_array(24);
    let mut cipher = aes_factory::init(Modes::ECB, Paddings::PKCS7);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
```
#### AES_CTR
```rust
    let plaintext: Vec<u8> = random_byte_array(24);
    let mut cipher = aes_factory::init(Modes::CTR, Paddings::PKCS7);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
```
#### AES_CBC
```rust
    let plaintext: Vec<u8> = random_byte_array(24);
    let mut cipher = aes_factory::init(Modes::CBC, Paddings::PKCS7);

    
    let ciphertext = cipher.encrypt(plaintext.clone());
    assert_eq!(plaintext, cipher.decrypt(ciphertext));
```

### RSA

```rust
    let rsa: RSA = RSA::init(1024);


    let plain = random_byte_array(12);
    let ciphertext = rsa.encrypt(plain.clone());

    let plaintext = rsa.decrypt(ciphertext.clone());
    assert_eq!(plain, plaintext);
```

### Kyber

```rust
    let cipher: Kyber512 = Kyber512::init();
    let plain = random_byte_array(32);
   
    let (u, v) = cipher.encrypt(plain.clone());
 
    assert_eq!(cipher.decrypt(u, v), plain);
```

### BFV
```rust
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
```
