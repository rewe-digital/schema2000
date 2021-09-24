use std::collections::HashSet;

#[derive(Debug)]
pub enum IntegerType {
    // enum covers both the "const" and the "enum" case: const is with only one element in HashSet
    Enum(HashSet<i64>),

    // after a certain size of the enum, it "flips over" and becomes a normal Integer
    Restriction,
}

impl IntegerType {
    pub fn new(i: i64) -> Self {
        let mut h = HashSet::new();
        h.insert(i);
        IntegerType::Enum(h)
    }
}
