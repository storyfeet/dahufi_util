pub struct DChar {
    pub cons: u32,
    pub vow: u32,
}

impl DChar {
    pub fn from_old(c: char) -> Option<DChar> {
        if (c as u32) < 0xe000 {
            return None;
        }
        let n = c as u32 - 0xe000;
        let cons = if n < 0xe054 {
            n % 14
        } else {
            15 + (n - 0xe054) % 14
        };
        let vow = (n / 14) % 6;
        Some(DChar { cons, vow })
    }
}
