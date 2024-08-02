use matrix::ByteMatrix;

pub mod block_cipher;
pub mod key;
mod matrix;
mod sbox;

const BLOCK_LEN_BIT: usize = 128;
const BLOCK_LEN_BYTE: usize = BLOCK_LEN_BIT / 8;
pub type Block = ByteMatrix<4, 4>;

fn xor<const L: usize>(a: [u8; L], b: [u8; L]) -> [u8; L] {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
