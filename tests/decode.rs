//! Decode tests
//!
//! Update the snapshots by running:
//!
//! ```sh
//! env UPDATE_EXPECT=1 cargo test
//! ```
//!
//! For more information see <https://docs.rs/expect-test>.

use expect_test::ExpectFile;
use std::process::{Command, Output};

fn doodle() -> Command {
    Command::new(env!("CARGO_BIN_EXE_doodle"))
}

#[track_caller]
fn check_output(output: Output, expected: ExpectFile) {
    if !output.status.success() {
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        panic!("command failed to execute. {}", output.status);
    }
    expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
}

mod jpg {
    use super::*;

    #[test]
    fn test_decode_test_jpg() {
        let output = doodle().args(["test.jpg"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.jpg.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test2_jpg() {
        let output = doodle().args(["test2.jpg"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test2.jpg.stdout");
        check_output(output, expected)
    }
}

mod png {
    use super::*;

    #[test]
    fn test_decode_test_png() {
        let output = doodle().args(["test.png"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.png.stdout");
        check_output(output, expected)
    }
}

mod riff {
    use super::*;

    #[test]
    fn test_decode_test_webp() {
        let output = doodle().args(["test.webp"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.webp.stdout");
        check_output(output, expected)
    }
}
