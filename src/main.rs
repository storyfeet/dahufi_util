use gobble::*;
use michuhu_lib::*;
use std::io::stdin;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut it = std::env::args().skip(1);

    let select = it.next();
    match select.as_ref().map(|s| &s[..]) {
        None => translate_stdin(),
        Some("words") => translate_words(it, false),
        Some("line") => translate_words(it, true),
        Some(_v) => Ok(()),
    }
}

fn translate_words<I: Iterator<Item = String>>(it: I, nl: bool) -> anyhow::Result<()> {
    let ps = chars_until(parser::Letter, eoi).map(|(a, _b)| a);
    let mut argy = false;
    for x in it {
        if argy {
            print!(" ")
        }
        let res = ps.parse_s(&x)?;
        print!("{}", res);
        argy = true;
    }
    if nl {
        println!("");
    }

    Ok(())
}

fn translate_stdin() -> anyhow::Result<()> {
    let mut r = stdin();
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    let res = chars_until(parser::Letter, eoi)
        .map(|(a, _b)| a)
        .parse_s(&s)?;
    print!("{}", res);
    Ok(())
}
