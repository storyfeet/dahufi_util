use gobble::*;
use michuhu_lib::*;
use std::io::stdin;
use std::io::Read;

use clap_conf::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut it = std::env::args().skip(1);

    let clp = clap_app!(michuhu =>
        (author:"Matthew Stoodley")
        (version:crate_version!())
        (about:"A Util program for working with the Michuhu language")
        (@arg files: -f --files +takes_value ... "The file locations")
        (@arg convert: -c --convert +takes_value ... "words to type out")
        //(@arg romanize: -r --romanize +takes_value ... "Michuhu to romanize")
        (@arg print_e: -e --print_e "Print the built dictionary engligh to michuhu")
        (@arg print_m: -m --print_m "print the built dictionart michuhu to english")
        (@arg noline: -n --no_line "no newline on end of output")
    )
    .get_matches();

    let conf = with_toml_env(&clp, &["home/.config/michuhu/conf.toml"]);

    if let Some(mul) = conf.grab_multi().arg("convert").done() {
        let mut sp = "";
        for w in mul {
            let res = parser::Converter.parse_s(&w)?;
            print!("{}{}", sp, res);
            sp = " ";
        }
        if !conf.bool_flag("noline", Filter::Arg) {
            println!("");
        }
    }

    Ok(())

    /*
    match select.as_ref().map(|s| &s[..]) {
        None => translate_stdin(),
        Some("words") => translate_words(it, false),
        Some("line") => translate_words(it, true),
        Some("get_e") => get_e(it),
        Some("build_e") => print_e_m(it),
        Some("build_m") => print_m_e(it),
        Some(_v) => Ok(()),
    }
    */
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
