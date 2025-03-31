use crate::format::BaseModule;
use doodle::{helper::*, Label, ValueType};
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
    let run_type_os = module.get_format_type(run.get_level()).clone();
    let rle_ascii_string_os = module.define_format_args(
        "rle.old-style.ascii-string",
        vec![(
            Label::Borrowed("runs"),
            ValueType::Seq(Box::new(run_type_os)),
        )],
        compute(flat_map(
            lambda("run", record_proj(var("run"), "buf")),
            var("runs"),
        )),
    );
    let old_style = module.define_format(
        "rle.old-style",
        record([
            ("runs", repeat(run.call())),
            ("data", rle_ascii_string_os.call_args(vec![var("runs")])),
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
        let run_type_ns = module.get_format_type(run.get_level()).clone();
        let rle_ascii_string_ns = module.define_format_args(
            "rle.new-style.ascii-string",
            vec![(
                Label::Borrowed("runs"),
                ValueType::Seq(Box::new(run_type_ns)),
            )],
            compute(flat_map(
                lambda("run", record_proj(var("run"), "buf")),
                var("runs"),
            )),
        );
        module.define_format(
            "rle.new-style",
            record_auto([
                ("_runs", repeat(run.call())),
                ("data", rle_ascii_string_ns.call_args(vec![var("_runs")])),
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
