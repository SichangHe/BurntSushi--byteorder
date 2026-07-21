use byteorder::{BigEndian, ByteOrder, LittleEndian};

fn model_be(n: u32) -> [u8; 4] {
    [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8]
}

fn model_le(n: u32) -> [u8; 4] {
    [n as u8, (n >> 8) as u8, (n >> 16) as u8, (n >> 24) as u8]
}

fn check(n: u32) {
    let be = model_be(n);
    let le = model_le(n);
    let mut actual = [0; 4];

    BigEndian::write_u32(&mut actual, n);
    assert_eq!(actual, be);
    assert_eq!(BigEndian::read_u32(&be), n);

    LittleEndian::write_u32(&mut actual, n);
    assert_eq!(actual, le);
    assert_eq!(LittleEndian::read_u32(&le), n);
}

#[test]
fn production_matches_verified_u32_model() {
    for half in 0..=u16::MAX {
        let half = half as u32;
        check(half);
        check(half << 16);
        check((half << 16) | ((!half) & 0xffff));
    }
}
