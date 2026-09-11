#![cfg(test)]

//! Phase 5 capstone (see `experiments/doodle-rec/PLAN.md` and `doc/RECURSION.md`): the decoder
//! functions in `super` (`Decoder_peano`/`Decoder_ping`/etc.) are real production codegen output,
//! frozen by `src/codegen/mod.rs`'s `#[ignore]`d `regenerate_recursion_fixture` test - re-run that
//! test (`cargo test --lib codegen::tests::regenerate_recursion_fixture -- --ignored`) after
//! changing the format definitions there, then re-run this file's own tests - for one
//! self-recursive format (`peano`) and one mutually-recursive pair (`ping`/`pong`). These tests
//! prove that output actually compiles as ordinary Rust and decodes real crafted bytes correctly
//! at recursion depth >= 2, plus that malformed input is cleanly rejected rather than panicking or
//! looping.

use super::*;

/// Walks a decoded `peano` back into the exact byte sequence it represents, to check correctness
/// without needing `PartialEq` on the generated type.
fn peano_bytes(v: &peano) -> Vec<u8> {
    match v {
        peano::Z(b) => vec![*b],
        peano::S(b, inner) => {
            let mut out = vec![*b];
            out.extend(peano_bytes(inner));
            out
        }
    }
}

/// Same as `peano_bytes`, for the `ping`/`pong` mutually-recursive pair. `pong` is a named
/// `Format::record` (see `experiments/doodle-rec/PLAN.md`'s Finding A/B) rather than a raw
/// `Tuple`, so its own back-reference to `ping` is a struct field, not a tuple position.
fn ping_bytes(v: &ping) -> Vec<u8> {
    match v {
        ping::Done(b) => vec![*b],
        ping::More(b, pong { tag, next }) => {
            let mut out = vec![*b, *tag];
            out.extend(ping_bytes(next));
            out
        }
    }
}

#[test]
fn peano_decodes_self_recursion_at_depth_2() {
    let data = *b"SSZ";
    let mut p = Parser::new(&data);
    let v = Decoder_peano(&mut p).unwrap();
    assert_eq!(peano_bytes(&v), data);
}

#[test]
fn peano_rejects_malformed_input() {
    let data = [0u8];
    let mut p = Parser::new(&data);
    assert!(
        Decoder_peano(&mut p).is_err(),
        "a byte that is neither 'Z' nor 'S' must be rejected, not silently accepted"
    );
}

#[test]
fn ping_pong_decodes_mutual_recursion_at_depth_2() {
    // ping --'A'--> pong --'B'--> ping --'Z'--> Done: two full round trips through the cycle.
    let data = *b"ABZ";
    let mut p = Parser::new(&data);
    let v = Decoder_ping(&mut p).unwrap();
    assert_eq!(ping_bytes(&v), data);
}

#[test]
fn ping_pong_rejects_malformed_input() {
    // pong's own prefix byte must be 'B'; anything else must be rejected, not silently accepted.
    let data = *b"AXZ";
    let mut p = Parser::new(&data);
    assert!(
        Decoder_ping(&mut p).is_err(),
        "pong's malformed prefix byte must be rejected, not silently accepted"
    );
}

#[test]
fn ping_rejects_malformed_first_byte() {
    let data = [0u8];
    let mut p = Parser::new(&data);
    assert!(
        Decoder_ping(&mut p).is_err(),
        "a byte that is neither 'Z' nor 'A' must be rejected, not silently accepted"
    );
}
