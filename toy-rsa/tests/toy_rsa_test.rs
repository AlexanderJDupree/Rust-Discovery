/// Test the toy_rsa library by generating multiple keys and ensuring that they
/// can be used to encrypt/decrypt randomized messages.
#[cfg(test)]
mod tests {
    use rand::Rng;
    use toy_rsa::*;

    #[test]
    fn test_encrypt_precomputed() {
        let public_key = 0xde9c5816141c8ba9;
        let message = 12345;
        assert_eq!(encrypt(public_key, message), 0x164e44b86776d497);
    }

    #[test]
    fn test_decrypt_precomputed() {
        let encrypted = 0x164e44b86776d497;
        let key = (0xed23e6cd, 0xf050a04d);
        let message = 12345;

        assert_eq!(decrypt(key, encrypted), message);
    }

    #[test]
    fn test_toy_rsa_randomized() {
        let mut rng = rand::thread_rng();

        // Take a thousand random samples, ensure we can encrypt/decrypt
        for _i in 0..1000 {
            let private_key = genkey();
            let public_key = (private_key.0 as u64) * (private_key.1 as u64);

            let message = rng.gen_range(0..u32::max_value());
            let encrypted = encrypt(public_key as u64, message);
            let decrypted = decrypt(private_key, encrypted);

            assert_eq!(message, decrypted);
        }
    }
}
