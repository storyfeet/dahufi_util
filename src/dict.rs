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
    pub a: String,
    pub extra: Option<String>,
}

#[derive(Debug)]
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
    pub fn insert(&mut self, r: Record) {
        let e_ans = match self.e_m.get(&r.english) {
            Some(Answer { a: mi, .. }) if mi == &r.michuhu => r.m_answer(),
            Some(Answer { a: mi, extra: ex }) => Answer {
                a: format!("{},{}", r.michuhu, mi),
                extra: r.extra.clone().or_else(|| ex.clone()),
            },
            None => r.m_answer(),
        };
        let m_ans = match self.m_e.get(&r.michuhu) {
            Some(Answer { a: en, .. }) if en == &r.english => r.e_answer(),
            Some(Answer { a: en, extra: ex }) => Answer {
                a: format!("{},{}", r.english, en),
                extra: r.extra.clone().or_else(|| ex.clone()),
            },
            None => r.e_answer(),
        };
        self.e_m.insert(r.english.clone(), e_ans);
        self.m_e.insert(r.michuhu.clone(), m_ans);
    }

    pub fn merge(&mut self, rhs: Self) {
        for (k, v) in rhs.e_m {
            self.e_m.insert(k, v);
        }
        for (k, v) in rhs.m_e {
            self.m_e.insert(k, v);
        }
    }
}
