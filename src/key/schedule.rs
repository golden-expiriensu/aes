use super::Word;

/// Round constant as defined in here https://en.wikipedia.org/wiki/AES_key_schedule.
pub(super) fn rcon(i: usize) -> Word {
    [rc(i), 0, 0, 0]
}

fn rc(i: usize) -> u8 {
    if i == 0 {
        panic!("Cannot derive round constant for an i = {i}")
    }
    if i == 1 {
        return 1;
    }

    let prev_rc = rc(i - 1);
    if prev_rc < 0x80 {
        2 * prev_rc
    } else {
        ((2 * prev_rc as u16) ^ 0x11B) as u8
    }
}

pub(super) fn rot_word(input: Word) -> Word {
    let mut input = input.clone();
    input.rotate_left(1);
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let values: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
        for (i, value) in values.into_iter().enumerate() {
            assert_eq!(value, rc(i + 1))
        }
    }
}
