use schema2000::render_schema;
use serde_json::{json, to_string_pretty, Value};

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

    let schema = schema2000::generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
      "type": "array",
      "items": {
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
          "value": {"anyOf": [{"type": "boolean"}, {"type": "string"}]}
        },
        "required": [
          "inputHint",
          "label",
          "name",
          "value"
        ],
        "type": "object"
      }
    });

    assert_eq!(schema_json, expected);
}

#[test]
fn test_single_object() {
    let document = json!([
      {
        "value": [
        {
          "id": 1
        }, {
          "name": "irgendwas"
        },
        "string",
        true,
        5
      ]
      }
    ]);

    let schema = schema2000::generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
      "type": "array",
      "items": {
        "properties": {
          "value": {
            "type": "array",
            "items": {
              "anyOf": [
                {"type": "boolean"},
                {"type": "integer"},
                {
                   "type": "object",
                    "properties": {
                      "id": { "type": "integer" },
                      "name": { "type": "string"}
                    },
                    "required": []
                },
                {"type": "string"}
              ]
            }
          }
        },
        "required": [
          "value"
        ],
        "type": "object"
      }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}

#[test]
fn test_single_nested_object() {
    let document = json!([
        {
           "value": "some string"
        },
        {
            "value": 42
        },
        {
            "value": {
                "a": "aaa"
            }
        },
        {
            "value": {
                "b": 111
            }
        }
    ]);

    let schema = schema2000::generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
          "type": "array",
          "items": {
              "type": "object",
              "properties": {
                  "value": {
                      "anyOf": [
                          {"type": "integer"},
                          {
                             "type": "object",
                              "properties": {
                                "a": { "type": "string" },
                                "b": { "type": "integer"}
                              },
                              "required": []
                          },
                          {"type": "string"}
                      ]
                  }
              },
              "required": ["value"]
          }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}

#[test]
fn test_array_merging() {
    let document = json!([[1], ["1"]]);

    let schema = schema2000::generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
          "type": "array",
          "items": {
              "type": "array",
              "items": {"anyOf": [
                {"type": "integer"},
                {"type": "string"}
            ]}
          }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}
