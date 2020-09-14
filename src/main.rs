use dahufi_lib::*;
use gobble::*;
use std::io::stdin;
use std::io::Read;

use clap_conf::prelude::*;

fn main() -> anyhow::Result<()> {
    let clp = clap_app!(michuhu =>
        (author:"Matthew Stoodley")
        (version:crate_version!())
        (about:"A Util program for working with the Michuhu language")
        (@arg files: -f --files +takes_value ... "The file locations")
        (@arg convert: -c --convert +takes_value ... "words to type out")
        (@arg translate: -t --translate + takes_value ... "Words to translate")
        //(@arg romanize: -r --romanize +takes_value ... "Michuhu to romanize")
        (@arg print_e: -e --print_e "Print the built dictionary engligh to michuhu")
        (@arg print_m: -m --print_m "print the built dictionart michuhu to english")
        (@arg noline: -n --no_line "no newline on end of output")
    )
    .get_matches();

    let conf = with_toml_env(&clp, &["{HOME}/.config/michuhu/conf.toml"]);

    if let Some(mul) = conf.grab_multi().arg("convert").done() {
        let mut sp = "";
        for w in mul {
            let res = parser::Converter.parse_s(&w).map_err(|e| e.strung())?;
            print!("{}{}", sp, res);
            sp = " ";
        }
        if !conf.bool_flag("noline", Filter::Arg) {
            println!("");
        }
        return Ok(());
    }

    let mut done_something = false;
    let d = build_dict(&conf)?;
    if conf.bool_flag("print_e", Filter::Arg) {
        done_something = true;
        print_e_m(&d);
    }
    if conf.bool_flag("print_m", Filter::Arg) {
        done_something = true;
        print_m_e(&d);
    }

    if let Some(v) = conf.grab_multi().arg("translate").done() {
        done_something = true;
        translate_words(&d, v);
    }

    if !done_something {
        convert_stdin()?;
    }
    Ok(())
}

fn print_e_m(mp: &dict::TwoWayMap) {
    for (k, v) in &mp.e_m.mp {
        println!("{} : {} ", k, v);
    }
}

fn print_m_e(mp: &dict::TwoWayMap) {
    for (k, v) in &mp.m_e.mp {
        println!("{} : {} ", k, v);
    }
}

fn build_dict<'a, G: Getter<'a, String>>(cfg: &'a G) -> anyhow::Result<dict::TwoWayMap> {
    let files = cfg.grab_multi().arg("files").conf("files").req()?;

    let mut res = dict::TwoWayMap::new();
    for fglob in files {
        //println!("Files = {:?}", fglob);
        let fglob = clap_conf::replace::replace_env(&fglob)?;
        for fname in glob::glob(&fglob)? {
            let mut s = String::new();
            let mut f = std::fs::File::open(fname?)?;
            f.read_to_string(&mut s)?;
            let lines = parser::Dict.parse_s(&s).map_err(|e| e.strung())?;
            res.merge(lines);
        }
    }
    Ok(res)
}

fn translate_words<I: Iterator<Item = S>, S: AsRef<str>>(mp: &dict::TwoWayMap, i: I) {
    for w in i {
        let mut not_found = false;
        match mp.e_m.mp.get(w.as_ref()) {
            Some(a) => println!("E: {} = {} ", w.as_ref(), a),
            None => not_found = true,
        }
        match mp.m_e.mp.get(w.as_ref()) {
            Some(a) => println!("M: {} = {} ", w.as_ref(), a),
            None => not_found &= not_found,
        }
        if not_found {
            println!("'{}' Not in dictionary", w.as_ref())
        }
    }
}

fn convert_stdin() -> anyhow::Result<()> {
    let mut r = stdin();
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    let res = parser::Converter.parse_s(&s).map_err(|e| e.strung())?;
    print!("{}", res);
    Ok(())
}
