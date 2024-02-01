# Rust Math and Cryptography library

## This is an ongoing project. It MUST NOT be used in production.
A Rust implementation of some SageMath functions.
It also includes Cryptographic primitives implemented with the same concepts of Google Tink, making schemes interchangable and keys easy to renew, swap or backup.
The aim is to put withing a single library all the currectly used schemes and some of the recently standardized PQ algorithms. 


## Cryptography
Symmetric ciphers:
- AES 128 

Modes of operation for AES:
- CTR_with_preprocessing, ECB, CBC

Padding Types:
- ANSI_X9_23
- ISO_10126
- PKCS7
- ISO_IEC_7816_4

Asymmetric ciphers:
- RSA
- Kyber512, Kyber768, Kyber1024

Homomorphic Encryption:
- BFV (Brakerski/Fan-Vercauteren)




## Mathematics
Numeric types:
- ZZ (Integers)
- QQ (Rational)
- RR (Real)

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

