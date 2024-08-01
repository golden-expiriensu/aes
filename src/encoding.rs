use std::slice::ChunksExact;

use crate::{sbox, Block, BLOCK_LEN_BYTE};

struct Encoder<'a> {
    src: ChunksExact<'a, u8>,
    dst: Vec<u8>,
}

impl<'a> Encoder<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self {
            src: data.chunks_exact(BLOCK_LEN_BYTE),
            dst: Vec::with_capacity(data.len()),
        }
    }

    //     fn encode_block(&mut self, mut round_keys: impl ExactSizeIterator<Item = Block>) {
    //         let block = self.src.next().unwrap();
    //         // Add round key
    //         let mut block = round_keys.next().unwrap().xor(block);
    //
    //         for _ in 0..round_keys.len() - 2 {
    //             block = sbox::substitute(block);
    //             shift_rows(&mut block);
    //             mix_columns(&mut block);
    //             block = round_keys.next().unwrap().xor(block)
    //         }
    //
    //         block = sbox::substitute(block);
    //         shift_rows(&mut block);
    //         self.dst
    //             .extend_from_slice(&round_keys.next().unwrap().xor(block));
    //     }
}

// fn shift_rows(block: &mut Block) {
//     assert_eq!(block.len(), 16);
//     //block.map(|e| e[0]);
//     block[4..8].rotate_left(1);
//     block[8..12].rotate_left(2);
//     block[12..16].rotate_left(3);
// }
//
// fn mix_columns(block: &mut Block) {
//     assert_eq!(block.len(), 16);
//     type Matrix = [[u8; 4]; 4];
//     const MATRIX: Matrix = [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];
//
//     let flip = |arr: [u8; 1]| arr[0];
//     let result: Matrix = [
//         matrix_mul_mod2e8(MATRIX, [[block[0]], [block[4]], [block[8]], [block[12]]]).map(flip),
//         matrix_mul_mod2e8(MATRIX, [[block[1]], [block[5]], [block[9]], [block[13]]]).map(flip),
//         matrix_mul_mod2e8(MATRIX, [[block[2]], [block[6]], [block[10]], [block[14]]]).map(flip),
//         matrix_mul_mod2e8(MATRIX, [[block[3]], [block[7]], [block[11]], [block[15]]]).map(flip),
//     ];
//
//     block.copy_from_slice(&result.concat());
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // Column major matrix to row major matrix. This is useful because theory operates on CM
//     // matrices but we store it in memory as row major because most operations are much easier to
//     // perform in row major.
//     fn cm2rm() {}
//
//     #[test]
//     fn test_shift_rows() {
//         let mut actual = [
//             0x20, 0x51, 0x15, 0x6a, 0x45, 0x26, 0x1b, 0xc0, 0xd0, 0x02, 0x02, 0x12, 0x9d, 0xf3,
//             0xa3, 0x67,
//         ];
//         shift_rows(&mut actual);
//
//         let expected = [
//             0x20, 0x51, 0x15, 0x6a, 0x26, 0x1b, 0xc0, 0x45, 0x02, 0x12, 0xd0, 0x02, 0x67, 0x9d,
//             0xf3, 0xa3,
//         ];
//         assert_eq!(expected, actual);
//     }
//
//     #[test]
//     fn test_mix_columns() {
//         let mut actual = [
//             0x20, 0x51, 0x15, 0x6a, 0x26, 0x1b, 0xc0, 0x45, 0x02, 0x12, 0xd0, 0x02, 0x67, 0x9d,
//             0xf3, 0xa3,
//         ];
//         mix_columns(&mut actual);
//
//         let expected = [
//             0x4f, 0x00, 0x52, 0xba, 0x0d, 0xcc, 0x16, 0x45, 0xab, 0xd2, 0x60, 0xd5, 0x8a, 0xdb,
//             0xd2, 0xa4,
//         ];
//         assert_eq!(expected, actual);
//     }
// }
