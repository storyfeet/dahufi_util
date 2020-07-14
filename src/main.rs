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
        Some("build_e") => print_e_m(it),
        Some("build_m") => print_m_e(it),
        Some(_v) => Ok(()),
    }
}

fn print_e_m<I: Iterator<Item = String>>(it: I) -> anyhow::Result<()> {
    let d = build_dict(it)?;
    for (k, v) in &d.e_m {
        if let Some(e) = &v.extra {
            println!("{} : {} ({})", k, v.a, e);
        } else {
            println!("{} : {} ", k, v.a);
        }
    }
    Ok(())
}

fn print_m_e<I: Iterator<Item = String>>(it: I) -> anyhow::Result<()> {
    let d = build_dict(it)?;
    for (k, v) in &d.m_e {
        if let Some(e) = &v.extra {
            println!("{} : {} ({})", k, v.a, e);
        } else {
            println!("{} : {} ", k, v.a);
        }
    }
    Ok(())
}

fn build_dict<I: Iterator<Item = String>>(it: I) -> anyhow::Result<dict::TwoWayMap> {
    let mut res = dict::TwoWayMap::new();
    for fname in it {
        let mut s = String::new();
        let mut f = std::fs::File::open(fname)?;
        f.read_to_string(&mut s)?;
        let lines = parser::Dict.parse_s(&s)?;
        res.merge(lines);
    }
    Ok(res)
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
