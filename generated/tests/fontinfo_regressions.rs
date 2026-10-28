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

/// Path of the font `$name` in the test-font corpus (see `TEST_OBJ_SUFFIX_FONT` in `.cargo/config.toml`).
///
/// Asserts that the file exists: `fontinfo` reports unreadable fonts but still exits successfully, so a
/// missing font would otherwise only show up as a snapshot mismatch (or, under `UPDATE_EXPECT=1`, as a
/// silently emptied snapshot).
macro_rules! fontfile {
    ( $name:expr ) => {{
        let path = concat!(
            env!("TEST_OBJ_ROOT"),
            "/",
            env!("TEST_OBJ_SUFFIX_FONT"),
            "/",
            $name
        );
        assert!(
            std::path::Path::new(path).is_file(),
            "missing font test-file: {path}"
        );
        path
    }};
}

macro_rules! testfile {
    ( $name:expr ) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/expected/fontinfo/",
            $name,
            ".stdout"
        )
    };
}

// ANCHOR - Klei.otf
#[test]
fn test_fontinfo_regression_klei() {
    let output = fontinfo()
        .args(["analyze", "-vv", fontfile!("Klei.otf")])
        .output()
        .unwrap();
    let expected = expect_test::expect_file!(testfile!("Klei.otf"));
    check_output(output, expected)
}

// ANCHOR - DroidSansArabic.ttf
#[test]
fn test_fontinfo_regression_droidsansarabic() {
    let output = fontinfo()
        .args(["analyze", "-vv", fontfile!("DroidSansArabic.ttf")])
        .output()
        .unwrap();
    let expected = expect_test::expect_file!(testfile!("DroidSansArabic.ttf"));
    check_output(output, expected)
}

// ANCHOR - Frankenpax.ttc
#[test]
fn test_fontinfo_regression_frankenpax() {
    let output = fontinfo()
        .args(["analyze", "-vv", fontfile!("Frankenpax.ttc")])
        .output()
        .unwrap();
    let expected = expect_test::expect_file!(testfile!("Frankenpax.ttc"));
    check_output(output, expected)
}

// ANNCHOR - SixtyfourConvergence-Regular-VariableFont_BLED,SCAN,XELA,YELA.ttf
#[test]
fn test_fontinfo_regression_sixtyfourconvergence() {
    let output = fontinfo()
        .args([
            "analyze",
            "-vv",
            fontfile!("SixtyfourConvergence-Regular-VariableFont_BLED,SCAN,XELA,YELA.ttf"),
        ])
        .output()
        .unwrap();
    let expected = expect_test::expect_file!(testfile!(
        "SixtyfourConvergence-Regular-VariableFont_BLED,SCAN,XELA,YELA.ttf"
    ));
    check_output(output, expected)
}
