//! Integration tests to ensure our `doodle::prelude` definitions are sound
//! and behave as expected.

use doodle::prelude;
use proptest::prelude::*;

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_succ_u8_max() {
    let _ = prelude::succ(u8::MAX);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_succ_u16_max() {
    let _ = prelude::succ(u16::MAX);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_succ_u32_max() {
    let _ = prelude::succ(u32::MAX);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_succ_u64_max() {
    let _ = prelude::succ(u64::MAX);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_pred_u8_zero() {
    let _ = prelude::pred(0u8);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_pred_u16_zero() {
    let _ = prelude::pred(0u16);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_pred_u32_zero() {
    let _ = prelude::pred(0u32);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_pred_u64_zero() {
    let _ = prelude::pred(0u64);
}

proptest! {
    #[test]
    fn test_succ_u8_nonmax(x in 0..u8::MAX) {
        prop_assert_eq!(prelude::succ(x), x + 1);
    }

    #[test]
    fn test_succ_u16_nonmax(x in 0..u16::MAX) {
        prop_assert_eq!(prelude::succ(x), x + 1);
    }

    #[test]
    fn test_succ_u32_nonmax(x in 0..u32::MAX) {
        prop_assert_eq!(prelude::succ(x), x + 1);
    }

    #[test]
    fn test_succ_u64_nonmax(x in 0..u64::MAX) {
        prop_assert_eq!(prelude::succ(x), x + 1);
    }

    #[test]
    fn test_pred_u8_nonzero(x in 1..=u8::MAX) {
        prop_assert_eq!(prelude::pred(x), x - 1);
    }

    #[test]
    fn test_pred_u16_nonzero(x in 1..=u16::MAX) {
        prop_assert_eq!(prelude::pred(x), x - 1);
    }

    #[test]
    fn test_pred_u32_nonzero(x in 1..=u32::MAX) {
        prop_assert_eq!(prelude::pred(x), x - 1);
    }

    #[test]
    fn test_pred_u64_nonzero(x in 1..=u64::MAX) {
        prop_assert_eq!(prelude::pred(x), x - 1);
    }
}
