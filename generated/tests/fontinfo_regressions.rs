//! `fontinfo` regression tests
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

fn fontinfo() -> Command {
    Command::new(env!("CARGO_BIN_EXE_fontinfo"))
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

macro_rules! fontfile {
    ( $name:expr ) => {
       concat!(env!("CARGO_MANIFEST_DIR"), "/../test-fonts/", $name)
    };
}

macro_rules! testfile {
    ( $name:expr ) => {
       concat!(env!("CARGO_MANIFEST_DIR"), "/tests/expected/fontinfo/", $name, ".stdout")
    };
}

// ANCHOR - Klei.otf
#[test]
fn test_fontinfo_regression_klei() {
    let output = fontinfo().args(["-vv", fontfile!("Klei.otf")]).output().unwrap();
    let expected = expect_test::expect_file!(testfile!("Klei.otf"));
    check_output(output, expected)
}

// ANCHOR - DroidSansArabic.ttf
#[test]
fn test_fontinfo_regression_droidsansarabic() {
    let output = fontinfo().args(["-vv", fontfile!("DroidSansArabic.ttf")]).output().unwrap();
    let expected = expect_test::expect_file!(testfile!("DroidSansArabic.ttf"));
    check_output(output, expected)
}


// ANCHOR - Frankenpax.ttc
#[test]
fn test_fontinfo_regression_frankenpax() {
    let output = fontinfo().args(["-vv", fontfile!("Frankenpax.ttc")]).output().unwrap();
    let expected = expect_test::expect_file!(testfile!("Frankenpax.ttc"));
    check_output(output, expected)
}
