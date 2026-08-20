#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

pub mod api_helper;
mod codegen_tests;

use doodle::prelude::*;
use doodle::try_sub;

/// expected size: 2
/// trait-unready: multiple (2) decoders exist (d#{0, 1})
#[derive(Debug, Copy, Clone)]
pub enum test_main {
    Fallback,
    InPermit(bool),
}

/// d#0
fn Decoder_test_main(input: &mut Parser<'_>) -> Result<test_main, ParseError> {
    Decoder1(input)
}

/// d#1
fn Decoder1(input: &mut Parser<'_>) -> Result<test_main, ParseError> {
    (|| {
        input.start_alt();
        let res = (|| {
            let inner = {
                let res = (|| Decoder2(input))();
                match res {
                    Ok(res) => res,

                    Err(err) => err.fallback_value(false, |err| {
                        log::error!("data-level parse error suppressed by Permit: {}", err);
                    })?,
                }
            };
            PResult::Ok(test_main::InPermit(inner))
        })();
        match res {
            Ok(inner) => {
                return PResult::Ok(inner);
            }

            Err(_e) => {
                input.next_alt(true)?;
            }
        };
        let res = (|| {
            let _ = (Decoder3(input))?;
            PResult::Ok(test_main::Fallback)
        })();
        match res {
            Ok(inner) => PResult::Ok(inner),

            Err(_e) => Err(_e),
        }
    })()
}

/// d#2
fn Decoder2(input: &mut Parser<'_>) -> Result<bool, ParseError> {
    {
        let sz = 6u8 as usize;
        input.start_slice(sz)?;
        let ret = ((|| {
            input.open_peek_context();
            let ret = ((|| {
                let ix0 = {
                    let b = input.read_byte()?;
                    if b == 97 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(13646096770106105413u64));
                    }
                };
                let ix1 = {
                    let b = input.read_byte()?;
                    if b == 98 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(2206609067086327257u64));
                    }
                };
                let ix2 = {
                    let b = input.read_byte()?;
                    if b == 99 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(11876854719037224982u64));
                    }
                };
                let ix3 = {
                    let b = input.read_byte()?;
                    if b == 100 {
                        b
                    } else {
                        return Err(ParseError::ExcludedBranch(18270091135093349626u64));
                    }
                };
                PResult::Ok(vec![ix0, ix1, ix2, ix3])
            })())?;
            // vvv MANUALLY COMMENTED OUT vvv
            // input.close_peek_context()?;
            PResult::Ok(ret)
        })())?;
        input.end_slice()?;
        ret
    };
    PResult::Ok(true)
}

/// d#3
fn Decoder3(_input: &mut Parser<'_>) -> Result<(), ParseError> {
    PResult::Ok(())
}
