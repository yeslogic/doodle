#![cfg(test)]

use crate::*;
use doodle::prelude::*;

#[test]
fn test_runtime_repeat() {
    let data = [0xAA, 0xBB, 0xBB, 0xAA, 0xBB, 0xBB, 0xCC];
    let mut p = Parser::new(&data);
    let _ = Decoder_test_outer(&mut p).unwrap();
}
