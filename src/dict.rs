use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub english: String,
    pub michuhu: String,
}

#[derive(Debug)]
pub enum Answer {
    One(String),
    Many(Vec<String>),
}

impl Answer {
    pub fn join(&mut self, b: Self) {
        let mut t = Answer::One(String::new());
        std::mem::swap(&mut t, self);
        use Answer::*;
        *self = match (t, b) {
            (One(o), Many(mut m)) | (Many(mut m), One(o)) => {
                if !m.contains(&o) {
                    m.push(o);
                }
                Many(m)
            }
            (Many(mut ma), Many(mb)) => {
                for o in mb {
                    if !ma.contains(&o) {
                        ma.push(o);
                    }
                }
                Many(ma)
            }
            (One(a), One(b)) => {
                if a == b {
                    One(a)
                } else {
                    Many(vec![a, b])
                }
            }
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::One(s) => write!(f, "{}", s),
            Answer::Many(v) => {
                let mut coma = "";
                for x in v {
                    write!(f, "{}{}", x, coma)?;
                    coma = " , "
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct AnsMap {
    pub mp: BTreeMap<String, Answer>,
}

impl AnsMap {
    pub fn new() -> Self {
        AnsMap {
            mp: BTreeMap::new(),
        }
    }

    pub fn insert_s(&mut self, k: String, v: String) {
        self.insert(k, Answer::One(v))
    }

    pub fn insert(&mut self, k: String, v: Answer) {
        if let Some(gm) = self.mp.get_mut(&k) {
            gm.join(v)
        } else {
            self.mp.insert(k, v);
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
            e_m: AnsMap::new(),
            m_e: AnsMap::new(),
        }
    }
    pub fn insert(&mut self, r: Record) {
        self.e_m.insert_s(r.english.clone(), r.michuhu.clone());
        self.m_e.insert_s(r.michuhu, r.english);
    }

    pub fn merge(&mut self, rhs: Self) {
        for (k, v) in rhs.e_m.mp {
            self.e_m.insert(k, v);
        }
        for (k, v) in rhs.m_e.mp {
            self.m_e.insert(k, v);
        }
    }
}
