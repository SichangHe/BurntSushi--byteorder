# Verified endian slices

## `u32`

`endian_u32.rs` models the four-byte component implemented by
`BigEndian::{read_u32, write_u32}` and
`LittleEndian::{read_u32, write_u32}` in `src/lib.rs`.

The executable model exposes each output byte explicitly. Its specifications
define decoding as the corresponding base-256 positional value. Verus proves
that both writers satisfy those definitions, both readers return those values,
and each read-after-write round trip returns the original `u32` for every input.
`tests/u32_verus_correspondence.rs` checks the production methods against the
same byte equations in 196,608 structured calls, including all-zero and
all-one halves, alternating halves, and every value representable in 16 bits.

Verifier provenance:

- version: `0.2026.03.17.a96bad0`
- SHA-256: `9a675a675ad2e37e9d51d1fadcf3e2d8377e413f7d915665d629e30e5d92c6ed`
- `staged_in_artifact_root=false`

The complete 783 MiB verifier closure already exists on the same filesystem,
so this document records its exact provenance instead of duplicating it.

Verify with that exact binary:

```text
/ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus --no-cheating verus/endian_u32.rs
```

## `u24`

`endian_u24.rs` models the three-byte component implemented by the public
`BigEndian::{read_u24, write_u24}` and
`LittleEndian::{read_u24, write_u24}` defaults in `src/lib.rs`. Those defaults
traverse the endian-specific variable-width `read_uint` and `write_uint`
implementations with a width of three bytes.

The executable shadow model proves exact base-256 byte equations for every
three-byte input and proves both read-after-write round trips for every
representable unsigned 24-bit value. Its writer precondition records the
production API's representability requirement. This is not a verifier-checked
refinement proof of the production crate: `tests/u24_verus_correspondence.rs`
provides focused correspondence evidence for the production methods, including
the accepted maximum, first rejected value, exact layouts, exhaustive reader
inputs, and preservation after the written three-byte prefix.

Verify with the same binary and provenance recorded above:

```text
/ssd1/sichangheagent/work_logs/private_artifacts/midas_lex_12096/toolchain/verus-2026-03-17/verus --no-cheating verus/endian_u24.rs
```
