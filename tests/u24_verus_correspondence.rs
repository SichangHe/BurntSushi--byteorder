use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::panic::{catch_unwind, AssertUnwindSafe};

const U24_MAX: u32 = 0x00ff_ffff;

fn model_be(n: u32) -> [u8; 3] {
    [(n >> 16) as u8, (n >> 8) as u8, n as u8]
}

fn model_le(n: u32) -> [u8; 3] {
    [n as u8, (n >> 8) as u8, (n >> 16) as u8]
}

fn check_write_and_roundtrip(n: u32) {
    let mut be = [0, 0, 0, 0xa5, 0x5a];
    BigEndian::write_u24(&mut be, n);
    assert_eq!(be[..3], model_be(n));
    assert_eq!(be[3..], [0xa5, 0x5a]);
    assert_eq!(BigEndian::read_u24(&be), n);

    let mut le = [0, 0, 0, 0xa5, 0x5a];
    LittleEndian::write_u24(&mut le, n);
    assert_eq!(le[..3], model_le(n));
    assert_eq!(le[3..], [0xa5, 0x5a]);
    assert_eq!(LittleEndian::read_u24(&le), n);
}

#[test]
fn production_writers_match_verified_u24_equations() {
    for n in
        [0, 1, 0xff, 0x100, 0xffff, 0x01_0000, 0x55_aa55, 0xaa_55aa, U24_MAX]
    {
        check_write_and_roundtrip(n);
    }

    for byte in 0..=u8::MAX {
        let byte = byte as u32;
        check_write_and_roundtrip(byte);
        check_write_and_roundtrip(byte << 8);
        check_write_and_roundtrip(byte << 16);
    }
}

#[test]
fn production_readers_cover_every_three_byte_input() {
    for n in 0..=U24_MAX {
        assert_eq!(BigEndian::read_u24(&model_be(n)), n);
        assert_eq!(LittleEndian::read_u24(&model_le(n)), n);
    }
}

#[test]
fn production_writers_reject_first_value_above_u24() {
    for write in
        [BigEndian::write_u24 as fn(&mut [u8], u32), LittleEndian::write_u24]
    {
        let mut bytes = [0; 3];
        assert!(catch_unwind(AssertUnwindSafe(|| write(
            &mut bytes,
            U24_MAX + 1
        )))
        .is_err());
    }
}
