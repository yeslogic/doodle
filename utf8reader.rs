#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use doodle::prelude::*;

#[derive(Debug, Clone)]
enum Type0 {
    ascii(Vec<u8>),
    utf8(Vec<char>),
}

fn Decoder0<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
    Some(Decoder1(scope, input)?)
}

fn Decoder1<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Type0> {
    // Branch #0
    {
        let inner = Decoder2(scope, input)?;
        Type0::ascii(inner);
    }
    // Branch #1
    {
        let inner = Decoder3(scope, input)?;
        Type0::utf8(inner);
    }
    Some(Ok(()))
}

fn Decoder2<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<u8>> {
    let mut accum = Vec::new();
    while true {
        let matching_ix = {
            let lookahead = &mut input.clone();
            let b = lookahead.read_byte()?;
            if ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0]).contains(b) {
                1
            } else {
                0
            }
        };
        if matching_ix == 0 {
            break;
        } else {
            let next_elem = Decoder6(scope, input)?;
            accum.push(next_elem);
        }
    }
    Some(accum)
}

fn Decoder3<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<Vec<char>> {
    let mut accum = Vec::new();
    while true {
        let matching_ix = {
            let lookahead = &mut input.clone();
            let b = lookahead.read_byte()?;
            match b {
                tmp if
                    ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0]).contains(
                        tmp
                    )
                => {
                    0
                }

                tmp if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(tmp) => { 0 }

                224 => { 0 }

                tmp if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(tmp) => { 0 }

                237 => { 0 }

                tmp if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(tmp) => { 0 }

                240 => { 0 }

                tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(tmp) => { 0 }

                244 => { 0 }

                _other => {
                    unreachable!(r#"unexpected: {:?}"#, _other);
                }
            }
        };
        if matching_ix == 0 {
            let next_elem = Decoder4(scope, input)?;
            accum.push(next_elem);
        } else {
            break;
        }
    }
    Some(accum)
}

fn Decoder4<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<char> {
    let inner = {
        let tree_index = {
            let lookahead = &mut input.clone();
            let b = lookahead.read_byte()?;
            match b {
                tmp if
                    ByteSet::from_bits([18446744073709551615, 18446744073709551615, 0, 0]).contains(
                        tmp
                    )
                => {
                    0
                }

                tmp if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(tmp) => { 1 }

                224 => { 2 }

                tmp if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(tmp) => { 2 }

                237 => { 2 }

                tmp if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(tmp) => { 2 }

                240 => { 3 }

                tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(tmp) => { 3 }

                244 => { 3 }

                _other => {
                    unreachable!(r#"unexpected: {:?}"#, _other);
                }
            }
        };
        match tree_index {
            0 => {
                let inner = {
                    let b = input.read_byte()?;
                    if
                        ByteSet::from_bits([
                            18446744073709551615, 18446744073709551615, 0, 0,
                        ]).contains(b)
                    {
                        b
                    } else {
                        return None;
                    }
                };
                (|byte: u8| byte as u32)(inner)
            }

            1 => {
                let inner = {
                    let field0 = {
                        let inner = {
                            let b = input.read_byte()?;
                            if ByteSet::from_bits([0, 0, 0, 4294967292]).contains(b) {
                                b
                            } else {
                                return None;
                            }
                        };
                        (|raw: u8| raw & 31)(inner)
                    };
                    let field1 = { Decoder5(scope, input)? };
                    (field0, field1)
                };
                (|bytes: (u8, u8)| {
                    match bytes {
                        (x1, x0) => { ((x1 as u32) << 6) | (x0 as u32) }

                        _other => {
                            unreachable!(r#"unexpected: {:?}"#, _other);
                        }
                    }
                })(inner)
            }

            2 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut input.clone();
                        let b = lookahead.read_byte()?;
                        match b {
                            224 => { 0 }

                            tmp if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(tmp) => {
                                1
                            }

                            237 => { 2 }

                            tmp if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(tmp) => {
                                3
                            }

                            _other => {
                                unreachable!(r#"unexpected: {:?}"#, _other);
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if b == 224 {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 15)(inner)
                            };
                            let field1 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if
                                        ByteSet::from_bits([
                                            0, 0, 18446744069414584320, 0,
                                        ]).contains(b)
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 63)(inner)
                            };
                            let field2 = { Decoder5(scope, input)? };
                            (field0, field1, field2)
                        }

                        1 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 0, 35175782154240]).contains(b) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 15)(inner)
                            };
                            let field1 = { Decoder5(scope, input)? };
                            let field2 = { Decoder5(scope, input)? };
                            (field0, field1, field2)
                        }

                        2 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if b == 237 {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 15)(inner)
                            };
                            let field1 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 4294967295, 0]).contains(b) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 63)(inner)
                            };
                            let field2 = { Decoder5(scope, input)? };
                            (field0, field1, field2)
                        }

                        3 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 0, 211106232532992]).contains(b) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 15)(inner)
                            };
                            let field1 = { Decoder5(scope, input)? };
                            let field2 = { Decoder5(scope, input)? };
                            (field0, field1, field2)
                        }

                        _other => {
                            unreachable!(r#"unexpected: {:?}"#, _other);
                        }
                    }
                };
                (|bytes: (u8, u8, u8)| {
                    match bytes {
                        (x2, x1, x0) => { ((x2 as u32) << 12) | ((x1 as u32) << 6) | (x0 as u32) }

                        _other => {
                            unreachable!(r#"unexpected: {:?}"#, _other);
                        }
                    }
                })(inner)
            }

            3 => {
                let inner = {
                    let tree_index = {
                        let lookahead = &mut input.clone();
                        let b = lookahead.read_byte()?;
                        match b {
                            240 => { 0 }

                            tmp if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(tmp) => {
                                1
                            }

                            244 => { 2 }

                            _other => {
                                unreachable!(r#"unexpected: {:?}"#, _other);
                            }
                        }
                    };
                    match tree_index {
                        0 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if b == 240 {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 7)(inner)
                            };
                            let field1 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if
                                        ByteSet::from_bits([
                                            0, 0, 18446744073709486080, 0,
                                        ]).contains(b)
                                    {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 63)(inner)
                            };
                            let field2 = { Decoder5(scope, input)? };
                            let field3 = { Decoder5(scope, input)? };
                            (field0, field1, field2, field3)
                        }

                        1 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 0, 3940649673949184]).contains(b) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 7)(inner)
                            };
                            let field1 = { Decoder5(scope, input)? };
                            let field2 = { Decoder5(scope, input)? };
                            let field3 = { Decoder5(scope, input)? };
                            (field0, field1, field2, field3)
                        }

                        2 => {
                            let field0 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if b == 244 {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 7)(inner)
                            };
                            let field1 = {
                                let inner = {
                                    let b = input.read_byte()?;
                                    if ByteSet::from_bits([0, 0, 65535, 0]).contains(b) {
                                        b
                                    } else {
                                        return None;
                                    }
                                };
                                (|raw: u8| raw & 63)(inner)
                            };
                            let field2 = { Decoder5(scope, input)? };
                            let field3 = { Decoder5(scope, input)? };
                            (field0, field1, field2, field3)
                        }

                        _other => {
                            unreachable!(r#"unexpected: {:?}"#, _other);
                        }
                    }
                };
                (|bytes: (u8, u8, u8, u8)| {
                    match bytes {
                        (x3, x2, x1, x0) => {
                            ((x3 as u32) << 18) |
                                ((x2 as u32) << 12) |
                                ((x1 as u32) << 6) |
                                (x0 as u32)
                        }

                        _other => {
                            unreachable!(r#"unexpected: {:?}"#, _other);
                        }
                    }
                })(inner)
            }

            _other => {
                unreachable!(r#"unexpected: {:?}"#, _other);
            }
        }
    };
    Some((|codepoint: u32| char::from_u32(codepoint).unwrap())(inner))
}

fn Decoder5<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let inner = {
        let b = input.read_byte()?;
        if ByteSet::from_bits([0, 0, 18446744073709551615, 0]).contains(b) {
            b
        } else {
            return None;
        }
    };
    Some((|raw: u8| raw & 63)(inner))
}

fn Decoder6<'input>(scope: &mut Scope, input: &mut ParseCtxt<'input>) -> Option<u8> {
    let b = input.read_byte()?;
    Some(
        if ByteSet::from_bits([18446744069414594048, 18446744073709551615, 0, 0]).contains(b) {
            b
        } else {
            return None;
        }
    )
}
