#![cfg(test)]

use crate::*;
use doodle::prelude::*;
use std::borrow::Borrow;

#[test]
fn test_recursive_maybe() {
    // C -> C -> E
    let data = b"CCE";
    let mut p = Parser::new(data);
    let chain = Decoder_test_chain(&mut p).unwrap();
    let mut tags = Vec::new();
    let mut cur: Option<&test_chain> = Some(&chain);
    while let Some(c) = cur {
        tags.push(c.tag);
        // `Borrow` so this holds whether or not the element type is boxed
        cur = c.next.as_ref().map(|n| -> &test_chain { n.borrow() });
    }
    assert_eq!(tags, b"CCE");
}
