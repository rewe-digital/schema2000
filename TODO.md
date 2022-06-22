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

## Renderer
- [X] Prüfen, wie ein Schema für ein Array mit multiple types aussieht.
  - Ggf. `generate_array_map` in `json_schema_renderer` anpassen 
- [X] Prüfen, ob man `generate_integer` und `generate_string` "zusammenfassen" kann.

## Other
- [ ] Use Into<NodeType> instead of direct NodeTypes as parameters
- [ ] Dokumentieren, welche Features von JSON Schema wir erkennen und unterstützen
