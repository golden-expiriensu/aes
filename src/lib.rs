use matrix::ByteMatrix;

use crate::key::Key;

mod encoding;
mod key;
mod matrix;
mod sbox;

#[allow(unused_variables)]
pub fn encode(data: &[u8], key: [u8; 16]) -> Vec<u8> {
    let keys = Key::new(key).expand();
    panic!("");
}

const BLOCK_LEN_BIT: usize = 128;
const BLOCK_LEN_BYTE: usize = BLOCK_LEN_BIT / 8;
type Block = ByteMatrix<4, 4>;

fn xor<const L: usize>(a: [u8; L], b: [u8; L]) -> [u8; L] {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
