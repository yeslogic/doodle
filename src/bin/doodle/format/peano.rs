use doodle::{helper::*, Expr};
use doodle::{Format, FormatModule, FormatRef};

pub fn main(module: &mut FormatModule) -> FormatRef {
    let peano_number = module.define_format(
        "peano.decimal",
        Format::Map(
            Box::new(tuple([
                repeat_between(Expr::U16(0), Expr::U16(9), is_byte(b'S')),
                is_byte(b'Z'),
            ])),
            lambda("x", seq_length(Expr::TupleProj(Box::new(var("x")), 0))),
        ),
    );
    module.define_format("peano.sequence", repeat1(peano_number.call()))
}
