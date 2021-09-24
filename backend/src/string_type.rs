use std::collections::HashSet;

#[derive(Debug)]
pub enum StringType {
    // enum covers both the "const" and the "enum" case: const is with only one element in HashSet
    Enum(HashSet<String>),

    // after a certain size of the enum, it "flips over" and becomes a normal string
    Restriction,
}

impl StringType {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let mut h = HashSet::new();
        h.insert(s.into());
        StringType::Enum(h)
    }
}
