mod key;

use key::Key;

#[allow(unused_variables)]
pub fn encode(data: &[u8], key: [u8; 32]) -> Vec<u8> {
    let keys = Key::new(key).expand();
    panic!("");
}
