use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit};
use generic_array::{GenericArray, typenum::U16};

pub fn gctr(key: &[u8; 16], icb: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut counter: GenericArray<u8, U16> = GenericArray::clone_from_slice(icb);
    let mut result = vec![];

    for chunk in plaintext.chunks(16) {
        let mut block: GenericArray<u8, U16> = GenericArray::clone_from_slice(&[0u8; 16]);
        block[..chunk.len()].copy_from_slice(chunk);
        cipher.encrypt_block(&mut block);

        for (b, c) in block.iter_mut().zip(counter.iter()) {
            *b ^= *c;
        }

        result.extend_from_slice(&block[..chunk.len()]);

        for byte in counter.iter_mut().rev() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
    }

    result
}
