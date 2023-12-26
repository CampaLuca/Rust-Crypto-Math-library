# Rust_SageMath

## This is an ongoing project. It MUST NOT be used in production.
A Rust implementation of some SageMath functions.
It included also Cryptographic primitives implemented with the same concept of Google Tink. 


### TODO:
- Update Miller Rabin test with vector of values in order to reduce the complexity if under a certain upper bound (search for Miller Rabin Test on wikipedia).

- Add Probabilistic Primality Test Baillie -> 2psp and lucas primality test
- is_prime should use:
- Pocklington-Lehmer Test (better)
- APRCL
- ECPP
- Implementing trait Operand or Instance for Polynomials and Polynomial Ring Instances OKAY
- Implement add, sub, mul, div for Polynomials
- AsAny method not implemented for polynomials
- Add AES CTR without preprocessing
- Random generator for each number type
- Random bigdecimal to be implemented
- Generate random polynomials and distribution for implementing Kyber, Saber, Dilithium
- create trait for polynomials for zero poly and identity polynomial generation
- quotient with irreducible poly MUSt recognicze when it has not to perform any operation on the given poly
- add Barret and Montgomery modulo reduction and NTT