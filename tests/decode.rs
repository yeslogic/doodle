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

mod jpg {
    use super::*;

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
}

mod png {
    use super::*;

    #[test]
    fn test_decode_test_png() {
        let output = doodle().args(["test.png"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.png.stdout");
        expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
    }
}

mod riff {
    use super::*;

    #[test]
    fn test_decode_test_webp() {
        let output = doodle().args(["test.webp"]).output().unwrap();
        let expected = expect_test::expect_file!("expected/decode/test.webp.stdout");
        expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
    }
}

mod stl {
    use super::*;

    #[test]
    fn test_decode_cube_stl() {
        let output = doodle()
            .args(["cube.stl", "--format=stl"])
            .output()
            .unwrap();
        let expected = expect_test::expect_file!("expected/decode/cube.stl.stdout");
        expected.assert_eq(&String::from_utf8_lossy(&output.stdout));
    }
}
