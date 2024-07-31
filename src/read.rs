#[derive(Copy, Clone)]
pub struct ReadCtxt<'a> {
    pub input: &'a [u8],
    pub offset: usize,
}

impl<'a> ReadCtxt<'a> {
    pub fn new(input: &'a [u8]) -> ReadCtxt<'a> {
        let offset = 0;
        ReadCtxt { input, offset }
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.offset..]
    }

    pub fn read_byte(&self) -> Option<(u8, ReadCtxt<'a>)> {
        if self.offset < self.input.len() {
            let b = self.input[self.offset];
            Some((
                b,
                ReadCtxt {
                    input: self.input,
                    offset: self.offset + 1,
                },
            ))
        } else {
            None
        }
    }

    pub fn split_at(&self, n: usize) -> Option<(ReadCtxt<'a>, ReadCtxt<'a>)> {
        if self.offset + n <= self.input.len() {
            let fst = ReadCtxt {
                input: &self.input[..self.offset + n],
                offset: self.offset,
            };
            let snd = ReadCtxt {
                input: self.input,
                offset: self.offset + n,
            };
            Some((fst, snd))
        } else {
            None
        }
    }

    pub(crate) fn skip_remainder(&self) -> ReadCtxt<'a> {
        let offset = self.input.len();
        ReadCtxt {
            input: self.input,
            offset,
        }
    }
}
