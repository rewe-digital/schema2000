use std::collections::BTreeSet;
use std::iter::FromIterator;

use maplit::btreeset;

pub const MAX_VALUES: usize = 5;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueCollection<T: Clone + Ord> {
    values: Option<BTreeSet<T>>,
}

impl<T: Clone + Ord> ValueCollection<T> {
    pub fn new() -> Self {
        Self {
            values: Some(BTreeSet::new()),
        }
    }

    pub fn with_value(value: &T) -> Self {
        Self {
            values: Some(btreeset![value.clone()]),
        }
    }

    pub fn with_values(values: Vec<T>) -> Self {
        Self {
            values: Some(BTreeSet::from_iter(values)),
        }
    }

    pub fn empty_collection() -> Self {
        Self { values: None }
    }

    pub fn values(&self) -> Option<Vec<&T>> {
        self.values
            .as_ref()
            .map(|value_set| value_set.iter().collect())
    }

    pub fn merge(&self, other: &ValueCollection<T>) -> ValueCollection<T> {
        match (&self.values, &other.values) {
            (Some(sv), Some(ov)) => {
                let merged_values: Vec<T> = sv.union(ov).cloned().collect();
                if merged_values.len() > MAX_VALUES {
                    ValueCollection { values: None }
                } else {
                    ValueCollection::with_values(merged_values)
                }
            }
            _ => ValueCollection { values: None },
        }
    }
}

impl<T: Clone + Ord> Default for ValueCollection<T> {
    fn default() -> Self {
        ValueCollection::new()
    }
}
