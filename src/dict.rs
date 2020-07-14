use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Record {
    pub english: String,
    pub michuhu: String,
    pub extra: Option<String>,
}

impl Record {
    fn e_answer(&self) -> Answer {
        Answer {
            a: self.english.clone(),
            extra: self.extra.clone(),
        }
    }
    fn m_answer(&self) -> Answer {
        Answer {
            a: self.michuhu.clone(),
            extra: self.extra.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Answer {
    a: String,
    extra: Option<String>,
}

pub struct TwoWayMap {
    pub e_m: BTreeMap<String, Answer>,
    pub m_e: BTreeMap<String, Answer>,
}

impl TwoWayMap {
    pub fn new() -> Self {
        TwoWayMap {
            e_m: BTreeMap::new(),
            m_e: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, mut r: Record) -> bool {
        let mut res = false;
        if let Some(Answer { a: mi, .. }) = self.e_m.get(&r.english) {
            if mi != &r.michuhu {
                r.michuhu.push(',');
                r.michuhu.push_str(mi);
                res = true;
            }
        }
        if let Some(Answer { a: en, .. }) = self.m_e.get(&r.michuhu) {
            if en != &r.english {
                r.michuhu.push(',');
                r.michuhu.push_str(en);
                res = true;
            }
        }
        self.e_m.insert(r.english.clone(), r.m_answer());
        self.m_e.insert(r.michuhu.clone(), r.e_answer());
        res
    }
}
