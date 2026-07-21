use vstd::prelude::*;

verus! {

pub open spec fn be_value(b0: u8, b1: u8, b2: u8) -> u32 {
    ((b0 as u32) << 16) | ((b1 as u32) << 8) | b2 as u32
}

pub open spec fn le_value(b0: u8, b1: u8, b2: u8) -> u32 {
    b0 as u32 | ((b1 as u32) << 8) | ((b2 as u32) << 16)
}

pub fn write_u24_be(n: u32) -> (bytes: [u8; 3])
    requires
        n & 0xff00_0000 == 0,
    ensures
        bytes[0] == #[verifier::truncate] ((n >> 16) as u8),
        bytes[1] == #[verifier::truncate] ((n >> 8) as u8),
        bytes[2] == #[verifier::truncate] (n as u8),
        be_value(bytes[0], bytes[1], bytes[2]) == n,
{
    let b0 = #[verifier::truncate] ((n >> 16) as u8);
    let b1 = #[verifier::truncate] ((n >> 8) as u8);
    let b2 = #[verifier::truncate] (n as u8);
    assert(
        n & 0xff00_0000 == 0 ==> (
            (((#[verifier::truncate] ((n >> 16) as u8)) as u32) << 16)
                | (((#[verifier::truncate] ((n >> 8) as u8)) as u32) << 8)
                | ((#[verifier::truncate] (n as u8)) as u32)
                == n
        )
    ) by (bit_vector);
    [b0, b1, b2]
}

pub fn read_u24_be(bytes: [u8; 3]) -> (n: u32)
    ensures
        n == be_value(bytes[0], bytes[1], bytes[2]),
{
    let b0 = bytes[0];
    let b1 = bytes[1];
    let b2 = bytes[2];
    ((b0 as u32) << 16) | ((b1 as u32) << 8) | b2 as u32
}

pub fn write_u24_le(n: u32) -> (bytes: [u8; 3])
    requires
        n & 0xff00_0000 == 0,
    ensures
        bytes[0] == #[verifier::truncate] (n as u8),
        bytes[1] == #[verifier::truncate] ((n >> 8) as u8),
        bytes[2] == #[verifier::truncate] ((n >> 16) as u8),
        le_value(bytes[0], bytes[1], bytes[2]) == n,
{
    let b0 = #[verifier::truncate] (n as u8);
    let b1 = #[verifier::truncate] ((n >> 8) as u8);
    let b2 = #[verifier::truncate] ((n >> 16) as u8);
    assert(
        n & 0xff00_0000 == 0 ==> (
            ((#[verifier::truncate] (n as u8)) as u32)
                | (((#[verifier::truncate] ((n >> 8) as u8)) as u32) << 8)
                | (((#[verifier::truncate] ((n >> 16) as u8)) as u32) << 16)
                == n
        )
    ) by (bit_vector);
    [b0, b1, b2]
}

pub fn read_u24_le(bytes: [u8; 3]) -> (n: u32)
    ensures
        n == le_value(bytes[0], bytes[1], bytes[2]),
{
    let b0 = bytes[0];
    let b1 = bytes[1];
    let b2 = bytes[2];
    b0 as u32 | ((b1 as u32) << 8) | ((b2 as u32) << 16)
}

pub fn roundtrip_u24_be(n: u32) -> (decoded: u32)
    requires
        n & 0xff00_0000 == 0,
    ensures
        decoded == n,
{
    read_u24_be(write_u24_be(n))
}

pub fn roundtrip_u24_le(n: u32) -> (decoded: u32)
    requires
        n & 0xff00_0000 == 0,
    ensures
        decoded == n,
{
    read_u24_le(write_u24_le(n))
}

fn main() {}

}
