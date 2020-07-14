use gobble::*;
pub mod dict;
pub mod parser;

fn cons_str(n: u32) -> &'static str {
    match n {
        0 => "k",
        1 => "d",
        2 => "ch",
        3 => "s",
        4 => "y",
        5 => "h",
        6 => "f",
        7 => "w",
        8 => "m",
        9 => "j",
        10 => "b",
        11 => "n",
        12 => "th",
        13 => "fl",
        _ => "#",
    }
}

pub fn romanize_char(c: char) -> String {
    if !parser::MiChar.char_bool(c) {
        return c.to_string();
    }
    let n = c as u32 - 0xe000;
    let cons = n % 14;
    let vow = n / 14;
    let mut res = cons_str(cons).to_string();
    res.push_str(match vow {
        0 => "",
        1 => "a",
        2 => "i",
        3 => "o",
        4 => "u",
        _ => "#",
    });
    res
}

pub fn romanize_str(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if parser::MiChar.char_bool(c) {
            res.push_str(&romanize_char(c))
        } else {
            res.push(c)
        }
    }
    res
}
