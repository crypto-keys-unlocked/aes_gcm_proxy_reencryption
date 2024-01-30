use ghash::GHash;
use ghash::universal_hash::UniversalHash;
use aes_gcm::KeyInit;
use aes::cipher::generic_array::{GenericArray, typenum::U16};

pub fn compute_ghash(key: &[u8; 16], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
    let key = GenericArray::<u8, U16>::from_slice(key);

    let mut ghash = GHash::new(key);

    for chunk in aad.chunks(16) {
        let mut block = GenericArray::<u8, U16>::default(); 
        let chunk_len = chunk.len();
        block[..chunk_len].copy_from_slice(chunk);
        ghash.update(&[block]);
    }

    for chunk in ciphertext.chunks(16) {
        let mut block = GenericArray::<u8, U16>::default(); 
        let chunk_len = chunk.len();
        block[..chunk_len].copy_from_slice(chunk);
        ghash.update(&[block]);
    }

    let tag = ghash.finalize();
    tag.into()
}
