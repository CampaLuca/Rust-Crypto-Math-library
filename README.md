# Rust Math and Cryptography library

## This is an ongoing project. It MUST NOT be used in production.
A Rust implementation of some SageMath functions.
It also includes Cryptographic primitives implemented with the same concepts of Google Tink, making schemes interchangable and keys easy to renew, swap or backup.
The aim is to put withing a single library all the currectly used schemes and some of the recently standardized PQ algorithms. 


### TODO:
- Update Miller Rabin test with vector of values in order to reduce the complexity if under a certain upper bound (search for Miller Rabin Test on wikipedia).
- Add Probabilistic Primality Test Baillie -> 2psp and lucas primality test
- is_prime should use Pocklington-Lehmer Test (better)
- APRCL
- ECPP
- Implement add, sub, mul, div for Multivariate Polynomials
- Add AES CTR without preprocessing
- Random bigdecimal 
- create trait for polynomials for zero poly and identity polynomial generation
- add Barret and Montgomery modulo reduction and NTT
