mod gctr;
mod aes;
mod ghash;

#[cfg(test)]
mod tests {
    use super::gctr::gctr;
    use hex_literal::hex;
    use super::aes::decrypt;
    use super::aes::encrypt;
    use super::ghash::compute_ghash;
    use hex::encode as hex_encode;

    #[test]
    fn test_ghash() {
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let aad = hex!("123456789aabbccddeeff112233445566778899000");
        let ciphertext = hex!("112233445566778899aabbccddeeff00");

        let computed_tag = compute_ghash(&key, &aad, &ciphertext);
        println!("Tag: {:?}", hex_encode(computed_tag));
    }


    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32]; 
        let plaintext = b"Hello, world!";

        // Encrypt
        let (nonce, ciphertext) = encrypt(&key, plaintext).expect("Encryption failed");

        // Decrypt
        let decrypted_plaintext = decrypt(&key, &nonce, &ciphertext).expect("Decryption failed");

        assert_eq!(&decrypted_plaintext[..], plaintext);
    }
    #[test]
    fn test_gctr() {
        let key = hex!("00112233445566778899aabbccddeeff");
        let icb = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
        let plaintext = b"Example plaintext data";

        let ciphertext = gctr(&key, &icb, plaintext);

        println!("Ciphertext: {:?}", ciphertext);
    }
}
