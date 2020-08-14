use crate::dict;
use gobble::*;

parser! { (Converter ->String)
    chars_until(Letter, eoi).map(|(a, _b)| a)
}

parser! { (EnStringPos -> ())
    ((Alpha,NumDigit).iplus(),(Alpha,NumDigit," -/~").istar()).ig()
}
parser! { (MiStringPos -> ())
    ((MiChar,NumDigit).iplus(),(MiChar,NumDigit," -/~").istar()).ig()
}

parser! { (Extra->String)
    ("(",(Alpha,NumDigit,WS,MiChar).star(),")").map(|(_,s,_)|s)
}
parser! { (MiEntry->String)
    (string(MiStringPos),Extra).map(|(m,e)|format!("{} ({})",m.trim(),e.trim()))
}
parser! { (EnEntry->String)
    (string(EnStringPos),Extra).map(|(m,e)|format!("{} ({})",m.trim(),e.trim()))
}

parser! { (Record->dict::Record)
    or(
        (ws__(EnEntry),":" ,ws_(MiEntry) ,maybe(Extra)).map(|(english,_,michuhu,extra)| dict::Record{english,michuhu}),
        (ws__(MiEntry),":" ,ws_(EnEntry) ,maybe(Extra)).map(|(michuhu,_,english,extra)| dict::Record{english,michuhu})
    )
}

parser! { (RLine->Option<dict::Record>)
    (
        maybe(Record),
        Any.except("\n|").istar(),
        or("\n|".one().ig(),eoi),
    )
        .map(|(a,_,_)|a)
}

parser! { (Dict->dict::TwoWayMap)
    star_until_ig(RLine,eoi)
        .map(|v|{
            let mut res = dict::TwoWayMap::new();
            for i in v{
                if let Some(r) = i{
                    res.insert(r);
                }
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
        _ => 0xe001,
    }
}

fn vowel(c: char) -> u32 {
    match c {
        'a' => 14,
        'b' => 14 * 2,
        'c' => 14 * 3,
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
    or!("k","d","ch","c","s","y","h","fl","v","f","w","m","j","b","n","th","t","l")
}

parser! { (MVowel->char)
    or!('a','i','o','u')
}

char_bool!(MiChar, |c| c >= '' && c <= '');
