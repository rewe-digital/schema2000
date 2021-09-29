use crate::NodeType;

/// helper to build union of two vectors (when modelling sets with vector)
pub fn vector_union(xs: Vec<NodeType>, ys: Vec<NodeType>) -> Vec<NodeType> {
    ys.into_iter().fold(xs, |mut acc, y| {
        if !acc.contains(&y) {
            acc.push(y);
        }

        acc
    })
}

#[cfg(test)]
mod test {
    use crate::utils::vector_union;
    use crate::NodeType;

    #[test]
    fn both_empty() {
        let a: Vec<NodeType> = vec![];
        let b: Vec<NodeType> = vec![];

        assert_eq!(vector_union(a, b), vec![]);
    }

    #[test]
    fn both_distinct() {
        let a = vec![NodeType::Boolean];
        let b = vec![NodeType::String];

        assert_eq!(
            vector_union(a, b),
            vec![NodeType::Boolean, NodeType::String]
        );
    }

    #[test]
    fn additional_in_a() {
        let a = vec![NodeType::String, NodeType::Boolean];
        let b = vec![NodeType::String];

        assert_eq!(
            vector_union(a, b),
            vec![NodeType::String, NodeType::Boolean]
        );
    }

    #[test]
    fn additional_in_b() {
        let a = vec![NodeType::String];
        let b = vec![NodeType::String, NodeType::Boolean];

        assert_eq!(
            vector_union(a, b),
            vec![NodeType::String, NodeType::Boolean]
        );
    }

    #[test]
    fn additional_in_a_and_b() {
        let a = vec![NodeType::String, NodeType::Integer];
        let b = vec![NodeType::String, NodeType::Boolean];

        assert_eq!(
            vector_union(a, b),
            vec![NodeType::String, NodeType::Integer, NodeType::Boolean]
        );
    }
}
