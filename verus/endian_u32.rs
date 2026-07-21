use vstd::prelude::*;

verus! {

pub open spec fn be_value(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b0 as u32) << 24)
        | ((b1 as u32) << 16)
        | ((b2 as u32) << 8)
        | b3 as u32
}

pub open spec fn le_value(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    b0 as u32
        | ((b1 as u32) << 8)
        | ((b2 as u32) << 16)
        | ((b3 as u32) << 24)
}

pub fn write_u32_be(n: u32) -> (bytes: [u8; 4])
    ensures
        bytes[0] == #[verifier::truncate] ((n >> 24) as u8),
        bytes[1] == #[verifier::truncate] ((n >> 16) as u8),
        bytes[2] == #[verifier::truncate] ((n >> 8) as u8),
        bytes[3] == #[verifier::truncate] (n as u8),
        be_value(bytes[0], bytes[1], bytes[2], bytes[3]) == n,
{
    let b0 = #[verifier::truncate] ((n >> 24) as u8);
    let b1 = #[verifier::truncate] ((n >> 16) as u8);
    let b2 = #[verifier::truncate] ((n >> 8) as u8);
    let b3 = #[verifier::truncate] (n as u8);
    assert(
        (((#[verifier::truncate] ((n >> 24) as u8)) as u32) << 24)
            | (((#[verifier::truncate] ((n >> 16) as u8)) as u32) << 16)
            | (((#[verifier::truncate] ((n >> 8) as u8)) as u32) << 8)
            | ((#[verifier::truncate] (n as u8)) as u32)
            == n
    ) by (bit_vector);
    let bytes = [b0, b1, b2, b3];
    bytes
}

pub fn read_u32_be(bytes: [u8; 4]) -> (n: u32)
    ensures
        n == be_value(bytes[0], bytes[1], bytes[2], bytes[3]),
{
    let b0 = bytes[0];
    let b1 = bytes[1];
    let b2 = bytes[2];
    let b3 = bytes[3];
    let n = ((b0 as u32) << 24)
        | ((b1 as u32) << 16)
        | ((b2 as u32) << 8)
        | b3 as u32;
    n
}

pub fn write_u32_le(n: u32) -> (bytes: [u8; 4])
    ensures
        bytes[0] == #[verifier::truncate] (n as u8),
        bytes[1] == #[verifier::truncate] ((n >> 8) as u8),
        bytes[2] == #[verifier::truncate] ((n >> 16) as u8),
        bytes[3] == #[verifier::truncate] ((n >> 24) as u8),
        le_value(bytes[0], bytes[1], bytes[2], bytes[3]) == n,
{
    let b0 = #[verifier::truncate] (n as u8);
    let b1 = #[verifier::truncate] ((n >> 8) as u8);
    let b2 = #[verifier::truncate] ((n >> 16) as u8);
    let b3 = #[verifier::truncate] ((n >> 24) as u8);
    assert(
        ((#[verifier::truncate] (n as u8)) as u32)
            | (((#[verifier::truncate] ((n >> 8) as u8)) as u32) << 8)
            | (((#[verifier::truncate] ((n >> 16) as u8)) as u32) << 16)
            | (((#[verifier::truncate] ((n >> 24) as u8)) as u32) << 24)
            == n
    ) by (bit_vector);
    let bytes = [b0, b1, b2, b3];
    bytes
}

pub fn read_u32_le(bytes: [u8; 4]) -> (n: u32)
    ensures
        n == le_value(bytes[0], bytes[1], bytes[2], bytes[3]),
{
    let b0 = bytes[0];
    let b1 = bytes[1];
    let b2 = bytes[2];
    let b3 = bytes[3];
    let n = b0 as u32
        | ((b1 as u32) << 8)
        | ((b2 as u32) << 16)
        | ((b3 as u32) << 24);
    n
}

pub fn roundtrip_u32_be(n: u32) -> (decoded: u32)
    ensures
        decoded == n,
{
    read_u32_be(write_u32_be(n))
}

pub fn roundtrip_u32_le(n: u32) -> (decoded: u32)
    ensures
        decoded == n,
{
    read_u32_le(write_u32_le(n))
}

fn main() {}

}
