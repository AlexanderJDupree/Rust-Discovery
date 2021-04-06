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
    (4, 4)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
pub fn encrypt(_key: u64, _msg: u32) -> u64 {
    todo!();
}

/// Decrupt the ciphertext `msg` using the RSA private `key` and return the
/// resulting plaintext
pub fn decrypt(_key: (u32, u32), _msg: u64) -> u32 {
    todo!();
}

/// Compute `λ(pq)` where `λ` is Carmichael's totient function. From wikiepedia:
/// > Since n = pq, λ(n) = lcm(λ(p),λ(q)), and since p and q are prime,
/// > λ(p) = φ(p) = p − 1 and likewise λ(q) = q − 1. Hence λ(n) = lcm(p − 1, q − 1)
fn carmichael(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1)
}

#[cfg(test)]
mod tests {
    use crate::genkey;

    #[test]
    fn trivial_test() {
        assert_eq!(genkey().1, 4);
    }
}
