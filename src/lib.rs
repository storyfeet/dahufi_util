use wasm_bindgen::prelude::*;

use gobble::*;
pub mod dict;
pub mod parser;

#[cfg(feature = "wee_alloc")]
#[glocal_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn roman_cons(c: char) -> &'static str {
    let n = c as u32 - 0xe000;
    let cons = if n < 0xe054 {
        n % 14
    } else {
        15 + (n - 0xe054) % 14
    };
    match cons {
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
        14 => "",
        15 => "l",
        16 => "gn",
        17 => "bl",
        18 => "sh",

        _ => "#",
    }
}

pub fn romanize_char(c: char) -> String {
    if !parser::MiChar.char_bool(c) {
        return c.to_string();
    }
    let n = c as u32 - 0xe000;
    let vow = (n / 14) % 6;
    let mut res = roman_cons(c).to_string();
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

#[wasm_bindgen]
pub fn dahufize_str(s: &str) -> String {
    parser::Converter.parse_s(s).unwrap_or("".to_string())
}
