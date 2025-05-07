use crate::format::BaseModule;
use doodle::helper::*;
use doodle::{FormatModule, FormatRef};

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let run = module.define_format(
        "rle.old-style.run",
        record([
            ("len", base.u8()),
            ("char", base.u8()),
            ("buf", repeat_count(var("len"), compute(var("char")))),
        ]),
    );
    let old_style = module.define_format(
        "rle.old-style",
        record([
            ("runs", repeat(run.call())),
            (
                "data",
                mk_ascii_string(compute(flat_map(
                    lambda("run", record_proj(var("run"), "buf")),
                    var("runs"),
                ))),
            ),
        ]),
    );

    let new_style = {
        let run = module.define_format(
            "rle.new-style.run",
            record([
                ("_len", base.u8()),
                ("_char", base.u8()),
                ("buf", repeat_count(var("_len"), compute(var("_char")))),
            ]),
        );
        module.define_format(
            "rle.new-style",
            record_auto([
                ("_runs", repeat(run.call())),
                (
                    "data",
                    mk_ascii_string(compute(flat_map(
                        lambda("run", record_proj(var("run"), "buf")),
                        var("_runs"),
                    ))),
                ),
            ]),
        )
    };

    module.define_format(
        "rle.main",
        union([
            monad_seq(is_byte(0), fmt_variant("old-style", old_style.call())),
            monad_seq(is_byte(1), fmt_variant("new-style", new_style.call())),
        ]),
    )
}
