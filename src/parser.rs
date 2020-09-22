use crate::dict;
use gobble::*;

parser! { (Converter ->String)
    chars_until(Letter, eoi).map(|(a, _b)| a)
}

parser! { (EnStringPos -> ())
    ((Alpha,NumDigit).iplus(),(Alpha,NumDigit,BothAllow).istar()).ig()
}
parser! { (MiStringPos -> ())
    ((MiChar,NumDigit).iplus(),(MiChar,NumDigit,BothAllow).istar()).ig()
}

parser! { (Extra->String)
    ("(",(Alpha,NumDigit,WS,MiChar).star(),")").map(|(_,s,_)|s)
}
parser! { (MiEntry->String)
    (string(MiStringPos),maybe(Extra)).map(|(m,e_op)|
        match e_op {
            Some(ex)=>format!("{} ({})",m.trim(),ex.trim()),
            None=>m.trim().to_string(),
        }
    )
}
parser! { (EnEntry->String)
    (string(EnStringPos),maybe(Extra)).map(|(m,e_op)|
        match e_op {
            Some(ex)=>format!("{} ({})",m.trim(),ex.trim()),
            None=>m.trim().to_string(),
        }
    )
}

parser! { (Record->dict::Record)
    or(
        (ws_(EnEntry),":" ,ws_(MiEntry)).map(|(english,_,michuhu)| dict::Record{english,michuhu}),
        (ws_(MiEntry),":" ,ws_(EnEntry)).map(|(michuhu,_,english)| dict::Record{english,michuhu})
    )
}

parser! { (EmptyLine ->())
    (not("\n|").istar(),"\n|".one()).ig()
}

parser! { (RecordLine->dict::Record)
    middle(
        maybe(or_ig!("*",(NumDigit.star(),"."))),
        Record,
        (maybe(ws__(",")),"\n|".one()),
    )
}

parser! { (NextRecord->dict::Record)
    star_until(EmptyLine,RecordLine).map(|(_,v)|v)
}

parser! { (Dict->dict::TwoWayMap)
    (star(NextRecord),star(EmptyLine),ws_(eoi))
        .map(|(v,_,_)|{
            let mut res = dict::TwoWayMap::new();
            for r in v{
                res.insert(r);
            }
            res
       })
}

parser! {(Letter->char)
    or(MLetter,Any.one())
}

fn consonant(s: &str) -> u32 {
    match s {
        "k" => 0xe000,
        "d" => 0xe001,
        "ch" | "c" => 0xe002,
        "s" => 0xe003,
        "y" => 0xe004,
        "h" => 0xe005,
        "f" => 0xe006,
        "w" => 0xe007,
        "m" => 0xe008,
        "j" => 0xe009,
        "b" => 0xe00a,
        "n" => 0xe00b,
        "th" | "t" => 0xe00c,
        "fl" | "v" => 0xe00d,
        "l" => 0xe055,
        "ng" | "g" => 0xe056,
        "bl" => 0xe057,
        "sh" | "z" => 0xe058,
        _ => 0xe001,
    }
}

fn vowel(c: char) -> u32 {
    match c {
        'a' => 14,
        'i' => 14 * 2,
        'o' => 14 * 3,
        'u' => 14 * 4,
        _ => 0,
    }
}

parser! {( MLetter->char)
    or!(
        (MCons,maybe(MVowel)).map(|(k,vop)|{
            std::char::from_u32( consonant(k)+vop.map(|v|vowel(v)).unwrap_or(0)).unwrap_or('#')
        }),
        MVowel.map(|v| std::char::from_u32(0xe054 + vowel(v) ).unwrap_or('#'))
    )
}

parser! { (MCons->&'static str)
    or!("ng","ch","th","sh","fl","bl","g","k","d","c","s","y","h","v","f","w","m","j","b","n","t","l","z")
}

parser! { (MVowel->char)
    or!('a','i','o','u')
}

char_bool!(MiChar, |c| c >= '' && c <= '');
char_bool!(BothAllow, "'?&/ \t~-");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record_with_extra() {
        let s = "hello(greeting): ";
        let record = Record.parse_s(s).unwrap();
        assert_eq!(
            record,
            dict::Record {
                english: "hello (greeting)".to_string(),
                michuhu: "".to_string(),
            }
        );
    }
    #[test]
    fn test_multi_record_with_extra() {
        let s = "Way(Path):,
        Way(Method):,
        ";
        let v = star(NextRecord).parse_s(s).unwrap();
        assert_eq!(
            v,
            vec![
                dict::Record {
                    english: "Way (Path)".to_string(),
                    michuhu: "".to_string(),
                },
                dict::Record {
                    english: "Way (Method)".to_string(),
                    michuhu: "".to_string(),
                }
            ]
        );
    }
}
