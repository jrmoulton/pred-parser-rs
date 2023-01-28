use std::{fmt::Display, str::Chars};

#[derive(Debug)]
enum List {
    Digit(char),
    Minus { lhs: Box<List>, rhs: Box<List> },
    Plus { lhs: Box<List>, rhs: Box<List> },
    Error,
}
impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Digit(ch) => f.write_str(&ch.to_string()),
            List::Minus { lhs, rhs } => f.write_fmt(format_args!("({lhs}+{rhs})")),
            List::Plus { lhs, rhs } => f.write_fmt(format_args!("({lhs}-{rhs})")),
            List::Error => f.write_str("Error"),
        }
    }
}

fn main() {
    let test_str = "-+9-a+127";

    println!("{}", build(&mut test_str.chars()))
}

fn build(chs: &mut Chars) -> List {
    let ch = chs.next().unwrap();
    match ch {
        '+' => List::Plus {
            lhs: Box::new(build(chs)),
            rhs: Box::new(build(chs)),
        },
        '-' => List::Minus {
            lhs: Box::new(build(chs)),
            rhs: Box::new(build(chs)),
        },
        '0'..='9' => List::Digit(ch),
        _ => List::Error,
    }
}
