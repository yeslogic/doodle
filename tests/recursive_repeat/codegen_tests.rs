#![cfg(test)]

use crate::*;
use doodle::prelude::*;
use std::borrow::Borrow;

#[test]
fn test_recursive_repeat() {
    // N(L, N(L, L))
    let data = b"NLNLL";
    let mut p = Parser::new(data);
    let tree = Decoder_test_tree(&mut p).unwrap();
    match tree {
        test_tree::Node(b'N', children) => {
            assert_eq!(children.len(), 2);
            // `Borrow` so this holds whether or not the element type is boxed
            let (c0, c1): (&test_tree, &test_tree) = (children[0].borrow(), children[1].borrow());
            assert!(matches!(c0, test_tree::Leaf(b'L')));
            assert!(matches!(c1, test_tree::Node(b'N', _)));
        }
        other => panic!("expected Node, got {other:?}"),
    }
}
