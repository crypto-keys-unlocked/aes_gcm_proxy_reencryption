use aes_gcm::{Aes256Gcm, aead::{Aead, generic_array::GenericArray}, KeyInit};
use rand::Rng;

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
        let key = GenericArray::from_slice(key);
            let cipher = Aes256Gcm::new(key);
        
            let nonce = rand::thread_rng().gen::<[u8; 12]>();
            let nonce_array = GenericArray::from_slice(&nonce);
        
            let ciphertext = cipher.encrypt(nonce_array, plaintext.as_ref())?;
        
            Ok((nonce.to_vec(), ciphertext))
        }
        
pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let key: &GenericArray<u8, _> = GenericArray::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(nonce);
    
        cipher.decrypt(nonce, ciphertext.as_ref())
    }
            