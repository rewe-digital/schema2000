use std::collections::BTreeSet;

pub enum SetVariances<'a, T> {
    Empty,
    OneElement(&'a T),
    Multiple(&'a BTreeSet<T>),
}

impl<'a, T> SetVariances<'a, T> {
    pub fn new(s: &'a BTreeSet<T>) -> Self {
        match s.len() {
            0 => Self::Empty,
            1 => Self::OneElement(s.iter().next().unwrap()),
            _ => Self::Multiple(s),
        }
    }
}
