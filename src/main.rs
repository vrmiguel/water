use water::parser::{parse_function_import, parse_instruction};

fn main() {
    dbg!(parse_instruction("i32.const 5").unwrap());

    dbg!(parse_instruction("(i32.const 5)").unwrap());

    dbg!(parse_instruction("(local.set $idx)").unwrap());
    dbg!(
        parse_instruction("(local.set $idx (i32.const 5))")
            .unwrap()
    );

    let import_wat = r#"(import "console" "log" (func $log (param i32) (param i32)))"#;

    if let Err(err) = parse_function_import(import_wat) {
        println!("{}", stringify_error(import_wat, err));
    }

    fn stringify_error(
        input: &str,
        error: nom::Err<nom::error::VerboseError<&str>>,
    ) -> String {
        match error {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(error)
            | nom::Err::Failure(error) => {
                nom::error::convert_error(input, error)
            }
        }
    }
}
