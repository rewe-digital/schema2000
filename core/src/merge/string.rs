use crate::model::StringNode;

pub fn merge_string(s1: StringNode, s2: StringNode) -> StringNode {
    StringNode {
        values: s1.values.merge(&s2.values),
    }
}

#[cfg(test)]
mod test {
    use crate::merge::string::merge_string;
    use crate::model;
    use crate::model::{StringNode, ValueCollection};

    #[test]
    fn test_merge_string() {
        let a = StringNode::with_value("a");
        let b = StringNode::with_values(vec!["b", "c"]);

        assert_eq!(
            StringNode::with_values(vec!("a", "b", "c")),
            merge_string(a, b)
        )
    }

    #[test]
    fn test_merge_string_same_value() {
        let a = StringNode::with_value("a");
        let b = StringNode::with_values(vec!["a", "b"]);

        assert_eq!(StringNode::with_values(vec!("a", "b")), merge_string(a, b))
    }

    #[test]
    fn test_merge_string_with_many_values() {
        let owned_values = many_values(model::MAX_VALUES);
        let values = owned_values.iter().map(|v| v as &str).collect();

        let a = StringNode::with_values(values);
        let b = StringNode::with_value("b");

        assert_eq!(
            StringNode {
                values: ValueCollection::empty_collection()
            },
            merge_string(a, b)
        )
    }

    fn many_values(count: usize) -> Vec<String> {
        let mut values: Vec<String> = Vec::new();
        for i in 1..=count {
            values.push(format!("a{}", i))
        }

        values
    }
}
