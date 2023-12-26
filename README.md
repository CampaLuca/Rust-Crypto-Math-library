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
- Implement add, sub, mul, div for Multivariate Polynomials
- Add AES CTR without preprocessing
- Random bigdecimal 
- create trait for polynomials for zero poly and identity polynomial generation
- add Barret and Montgomery modulo reduction and NTT