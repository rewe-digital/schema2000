use renderer::render_schema;
use serde_json::{json, Value};

#[test]
fn test_distinct_object() {
    let document = json!([
      {
        "name": "BatchManagementRequirement",
        "value": false,
        "inputHint": "SINGLE_LINE",
        "label": {
          "de": "Batch Management Requirement",
          "en": "Batch Management Requirement"
        }
      },
      {
        "name": "Brand",
        "value": "MAGGI",
        "inputHint": "SINGLE_LINE",
        "label": {
          "en": "Brand",
          "de": "Marke (DSD)"
        }
      }
    ]);

    let schema = backend::generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&*result).unwrap();

    let expected = json!({
      "properties": {
        "inputHint": {
          "type": "string"
        },
        "label": {
          "properties": {
            "de": {
              "type": "string"
            },
            "en": {
              "type": "string"
            }
          },
          "required": [
            "de",
            "en"
          ],
          "type": "object"
        },
        "name": {
          "type": "string"
        },
        "value": {"anyOf": [{"type": "string"}, {"type": "boolean"}]}
      },
      "required": [
        "inputHint",
        "label",
        "name",
        "value"
      ],
      "type": "object"
    });

    assert_eq!(schema_json, expected);
}
