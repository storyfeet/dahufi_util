use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Record {
    pub english: String,
    pub michuhu: String,
}

pub enum Answer {
    One(String),
    Many(Vec<String>),
}

pub struct AnsMap {
    mp: BTreeMap<String, Answer>,
}

impl AnsMap {
    pub fn new() -> Self {
        AnsMap {
            mp: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self,k:String,v:String) -> {
        match self.get_mut(&k){
            Some(Answer::One(s))=
        }
    }
}

#[derive(Debug)]
pub struct TwoWayMap {
    pub e_m: AnsMap,
    pub m_e: AnsMap,
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
