use analytic_engine::printer::print_conversion;
use lalrpop_util::lalrpop_mod;
use linefeed::{Interface, ReadResult};

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub expr
);

#[cfg(test)]
mod tests {
    use super::*;
    use analytic_engine::printer::show_expr;

    #[test]
    fn parse() {
        let expr = expr::TreeParser::new().parse("1 + 2").unwrap();
        assert_eq!(&show_expr(&expr), "(1? + 2?)");
    }
}

pub fn main() -> std::io::Result<()> {
    let reader = Interface::new("analytic")?;
    let parser = expr::TreeParser::new();
    reader.set_prompt("α> ")?;
    while let ReadResult::Input(input) = reader.read_line()? {
        let interp = parser.parse(&input);
        match interp {
            Ok(expr) => print_conversion(&expr),
            Err(err) => eprintln!("[ERROR]: {}", err),
        }
    }
    println!("Analysis Complete.");
    Ok(())
}
