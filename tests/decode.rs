//! Decode tests
//!
//! Update the snapshots by running:
//!
//! ```sh
//! env UPDATE_EXPECT=1 cargo test
//! ```
//!
//! For more information see <https://docs.rs/expect-test>.

use std::process::Command;

fn doodle() -> Command {
    Command::new(env!("CARGO_BIN_EXE_doodle"))
}

#[test]
fn test_decode_test_png() {
    let output = doodle().args(["test.png"]).output().unwrap();
    let expected = expect_test::expect_file!("expected/decode/test.png.stdout");
    expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_decode_test_jpg() {
    let output = doodle().args(["test.jpg"]).output().unwrap();
    let expected = expect_test::expect_file!("expected/decode/test.jpg.stdout");
    expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_decode_test2_jpg() {
    let output = doodle().args(["test2.jpg"]).output().unwrap();
    let expected = expect_test::expect_file!("expected/decode/test2.jpg.stdout");
    expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
}
