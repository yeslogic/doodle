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

mod gif {
    use super::*;

    #[test]
    fn test_decode_test_gif() {
        let output = doodle().args(["file", "test.gif"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.gif.stdout");
        check_output(output, expected)
    }
}

mod jpeg {
    use super::*;

    #[test]
    fn test_decode_test_jpg() {
        let output = doodle().args(["file", "test.jpg"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.jpg.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test2_jpg() {
        let output = doodle().args(["file", "test2.jpg"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test2.jpg.stdout");
        check_output(output, expected)
    }
}

mod png {
    use super::*;

    #[test]
    fn test_decode_test_png() {
        let output = doodle().args(["file", "test.png"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.png.stdout");
        check_output(output, expected)
    }
}

mod riff {
    use super::*;

    #[test]
    fn test_decode_test_webp() {
        let output = doodle().args(["file", "test.webp"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.webp.stdout");
        check_output(output, expected)
    }
}

mod tar {
    use super::*;

    #[test]
    fn test_decode_test_tar() {
        let output = doodle().args(["file", "test.tar"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.tar.stdout");
        check_output(output, expected)
    }
}

mod text {
    use super::*;

    #[test]
    fn test_decode_test_text() {
        let output = doodle().args(["file", "test.txt"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.txt.stdout");
        check_output(output, expected)
    }
}

mod gzip {
    use super::*;

    #[test]
    fn test_decode_test1_gzip() {
        let output = doodle().args(["file", "test1.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test1.gz.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test2_gzip() {
        let output = doodle().args(["file", "test2.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test2.gz.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test3_gzip() {
        let output = doodle().args(["file", "test3.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test3.gz.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test4_gzip() {
        let output = doodle().args(["file", "test4.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test4.gz.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test5_gzip() {
        let output = doodle().args(["file", "test5.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test5.gz.stdout");
        check_output(output, expected)
    }

    #[test]
    fn test_decode_test6_gzip() {
        let output = doodle().args(["file", "test6.gz"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test6.gz.stdout");
        check_output(output, expected)
    }
}

mod utf8 {
    use super::*;

    #[test]
    fn test_decode_test_utf8() {
        let output = doodle().args(["file", "test.utf8"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.utf8.stdout");
        check_output(output, expected)
    }
}
