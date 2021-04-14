//! Crate provides RSA key generation, encryption, and decryption. This crate is
//! designed to as a *TOY* exercise and *SHOULD NOT* be used for any real
//! cryptographic activites.
//!
//! Alexander DuPree 2021
use toy_rsa_lib::*;

/// Fixed RSA Encryption exponent
pub const EXP: u64 = 65537;

/// Generate a pair of primnes in the range `2**30..2**31` suitable for RSA
/// encryption with exponent [`EXP`]. **Warning**: this routine has unbounded
/// runtime; it works by generate-and-test, generating pairs of primes `p` `q`
/// and testing that they satisfy `λ(pq) <= EXP` and that `λ(pq)` has no common
/// factors with `EXP`
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime();
        let q = rsa_prime();
        let lambda = lcm((p - 1) as u64, (q - 1) as u64);
        if EXP < lambda && gcd(EXP, lambda) == 1 {
            return (p, q);
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the ciphertext `msg` using the RSA private `key` and return the
/// resulting plaintext
pub fn decrypt((p, q): (u32, u32), msg: u64) -> u32 {
    assert!(p != 0 && q != 0);

    let key = (p as u64) * (q as u64);
    let lambda = lcm((p - 1) as u64, (q - 1) as u64);
    let d = modinverse(EXP, lambda);

    modexp(msg, d, key) as u32
}
