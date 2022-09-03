use water::parser::parse_instruction;

fn main() {
    dbg!(parse_instruction("i32.const 5").unwrap());

    dbg!(parse_instruction("(i32.const 5)").unwrap());

    dbg!(parse_instruction("(local.set $idx)").unwrap());
    dbg!(parse_instruction("(local.set $idx (i32.const 5))").unwrap());
}
