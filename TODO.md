# TODO

## Prio
- [X] `merge_string` aus `merge/mod.rs` extrahieren
- [X] `generate_node_type_for_array_values` in `generate.rs` refactorisieren.
  - Lukas weiß wie
- [X] `StringNode` in `generate_node_type_for_array_values` berücksichtigen
  - Test ergänzen?
- [X] fix `test_merge_different_types`
  - Move `merge_node_types_to_any` an bessere Stelle. (merge?)
  - Nutze das in `merge_any`

## Other
- [ ] Use Into<NodeType> instead of direct NodeTypes as parameters
