#![cfg(test)]
extern crate log;
extern crate stderrlog;

use stderrlog::StdErrLog;

use super::*;

#[test]
fn permit_state_error() -> Result<(), Box<dyn 'static + std::error::Error + Send + Sync>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Debug)
        .init()
        .unwrap();
    let input = b"abcdefghi";
    let mut parser = Parser::new(input);
    let res = Decoder_test_main(&mut parser);
    match res {
        Ok(test_main::Fallback) => Ok(()),
        Ok(test_main::InPermit(true)) => {
            panic!("expected state error, but parse unexpectedly succeeded")
        }
        Ok(test_main::InPermit(false)) => {
            panic!("expected state error, but permit caught the state-error")
        }
        Err(e) => Err(e.into()),
    }
}
