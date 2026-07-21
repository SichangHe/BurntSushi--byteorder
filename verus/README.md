# Verified `u32` endian slice

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
