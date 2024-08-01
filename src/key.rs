use crate::{sbox, xor, Block, BLOCK_LEN_BYTE};

mod schedule;

const WORD_LEN_BIT: usize = 32;
const WORD_LEN_BYTE: usize = WORD_LEN_BIT / 8;

type Word = [u8; WORD_LEN_BYTE];

pub(crate) struct Key<const L: usize> {
    inner: [u8; L],
    n_rounds: usize,
    n_words: usize,
}

impl<const L: usize> Key<L> {
    pub(crate) const fn new(key: [u8; L]) -> Self {
        let n_rounds = match L {
            16 => 10,
            24 => 12,
            32 => 14,
            _ => panic!("Supported key lengths: 16, 24 and 32 byte"),
        };

        Self {
            inner: key,
            n_rounds,
            n_words: L / WORD_LEN_BYTE,
        }
    }

    pub(crate) fn expand(self) -> Vec<Block> {
        let orig_key = self
            .inner
            .chunks_exact(self.n_words)
            .map(Word::try_from)
            .map(|r| r.unwrap())
            .collect::<Vec<Word>>();

        // AES requires a separate 128-bit round key block for each round plus one more.
        let n_words = self.n_words * (self.n_rounds + 1);
        let mut words = Vec::with_capacity(n_words);

        for i in 0..n_words {
            let word = if i < self.n_words {
                orig_key[i]
            } else {
                if i % self.n_words == 0 {
                    xor(
                        words[i - self.n_words],
                        xor(
                            schedule::rot_word(words[i - 1]).map(sbox::sub_byte),
                            schedule::rcon(i / self.n_words),
                        ),
                    )
                } else if self.n_words > 6 && i % self.n_words == 4 {
                    xor(words[i - self.n_words], words[i - 1].map(sbox::sub_byte))
                } else {
                    xor(words[i - self.n_words], words[i - 1])
                }
            };
            words.push(word);
        }

        words
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .chunks_exact(BLOCK_LEN_BYTE)
            .map(|chunk| Block::try_from(chunk).unwrap())
            .collect::<Vec<Block>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let key = [
            0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20,
            0x46, 0x75,
        ];

        let rounds = [
            [
                0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20,
                0x46, 0x75,
            ],
            [
                0xE2, 0x32, 0xFC, 0xF1, 0x91, 0x12, 0x91, 0x88, 0xB1, 0x59, 0xE4, 0xE6, 0xD6, 0x79,
                0xA2, 0x93,
            ],
            [
                0x56, 0x08, 0x20, 0x07, 0xC7, 0x1A, 0xB1, 0x8F, 0x76, 0x43, 0x55, 0x69, 0xA0, 0x3A,
                0xF7, 0xFA,
            ],
            [
                0xD2, 0x60, 0x0D, 0xE7, 0x15, 0x7A, 0xBC, 0x68, 0x63, 0x39, 0xE9, 0x01, 0xC3, 0x03,
                0x1E, 0xFB,
            ],
            [
                0xA1, 0x12, 0x02, 0xC9, 0xB4, 0x68, 0xBE, 0xA1, 0xD7, 0x51, 0x57, 0xA0, 0x14, 0x52,
                0x49, 0x5B,
            ],
            [
                0xB1, 0x29, 0x3B, 0x33, 0x05, 0x41, 0x85, 0x92, 0xD2, 0x10, 0xD2, 0x32, 0xC6, 0x42,
                0x9B, 0x69,
            ],
            [
                0xBD, 0x3D, 0xC2, 0x87, 0xB8, 0x7C, 0x47, 0x15, 0x6A, 0x6C, 0x95, 0x27, 0xAC, 0x2E,
                0x0E, 0x4E,
            ],
            [
                0xCC, 0x96, 0xED, 0x16, 0x74, 0xEA, 0xAA, 0x03, 0x1E, 0x86, 0x3F, 0x24, 0xB2, 0xA8,
                0x31, 0x6A,
            ],
            [
                0x8E, 0x51, 0xEF, 0x21, 0xFA, 0xBB, 0x45, 0x22, 0xE4, 0x3D, 0x7A, 0x06, 0x56, 0x95,
                0x4B, 0x6C,
            ],
            [
                0xBF, 0xE2, 0xBF, 0x90, 0x45, 0x59, 0xFA, 0xB2, 0xA1, 0x64, 0x80, 0xB4, 0xF7, 0xF1,
                0xCB, 0xD8,
            ],
            [
                0x28, 0xFD, 0xDE, 0xF8, 0x6D, 0xA4, 0x24, 0x4A, 0xCC, 0xC0, 0xA4, 0xFE, 0x3B, 0x31,
                0x6F, 0x26,
            ],
        ];

        let round_keys = Key::<16>::new(key).expand();
        assert_eq!(rounds.len(), round_keys.len());

        for (i, (actual, expected)) in round_keys
            .into_iter()
            .zip(rounds.into_iter())
            .enumerate()
            .collect::<Vec<_>>()
        {
            assert_eq!(
                Block::try_from(expected).unwrap(),
                actual,
                "Invalid schedule for {i}th round"
            );
        }
    }
}
