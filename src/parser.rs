use crate::dict;
use gobble::*;

parser! { (EnString -> String)
    string(((Alpha,NumDigit).iplus(),(Alpha,NumDigit," -").istar())).map(|s|s.trim().to_string())
}

parser! { (MiString->String)
    string(((MiChar).iplus(),(MiChar," -").istar())).map(|s|s.trim().to_string())
}

parser! { (Extra->String)
    ("(",(Alpha,NumDigit,WS,MiChar).star(),")").map(|(_,b,_)|b)
}

parser! { (Record->dict::Record)
    or(
        (ws_(EnString),":" ,ws_(MiString) ,maybe(Extra)).map(|(english,_,michuhu,extra)| dict::Record{english,michuhu,extra}),
        (ws_(MiString),":" ,ws_(EnString) ,maybe(Extra)).map(|(michuhu,_,english,extra)| dict::Record{english,michuhu,extra})
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
    repeat_until_ig(RLine,eoi)
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

parser! {( MLetter->char)
    (MCons,maybe(MVowel)).map(|(k,vop)|{
        std::char::from_u32(match k {
            "k"=> 0xe000,
            "d"=> 0xe001,
            "ch"|"c"=> 0xe002,
            "s"=> 0xe003,
            "y"=> 0xe004,
            "h"=> 0xe005,
            "f"=> 0xe006,
            "w"=> 0xe007,
            "m"=> 0xe008,
            "j"=> 0xe009,
            "b"=> 0xe00a,
            "n"=> 0xe00b,
            "th"|"t"=> 0xe00c,
            "fl"|"v"=> 0xe00d,
            _=>0xe001,
        } + match vop {
            Some('a')=>14,
            Some('i')=>14*2,
            Some('o')=>14*3,
            Some('u')=>14*4,
            _=>0,

        }).unwrap_or('#')
    })
}

parser! { (MCons->&'static str)
    or!("k","d","ch","c","s","y","h","fl","v","f","w","m","j","b","n","th","t")
}

parser! { (MVowel->char)
    or!('a','i','o','u')
}

char_bool!(MiChar, |c| c >= '' && c <= '');
