use gonfig::merge::{ConfigMerger, MergeStrategy};
use serde_json::json;

#[test]
fn test_deep_merge() {
    let merger = ConfigMerger::new(MergeStrategy::Deep);

    let base = json!({
        "database": {
            "host": "localhost",
            "port": 5432
        },
        "logging": {
            "level": "info"
        }
    });

    let incoming = json!({
        "database": {
            "port": 3306,
            "username": "admin"
        },
        "logging": {
            "format": "json"
        }
    });

    let result = merger.merge_sources(vec![(base, 1), (incoming, 2)]);

    assert_eq!(result["database"]["host"], "localhost");
    assert_eq!(result["database"]["port"], 3306);
    assert_eq!(result["database"]["username"], "admin");
    assert_eq!(result["logging"]["level"], "info");
    assert_eq!(result["logging"]["format"], "json");
}

#[test]
fn test_replace_merge() {
    let merger = ConfigMerger::new(MergeStrategy::Replace);

    let base = json!({
        "database": {
            "host": "localhost",
            "port": 5432
        },
        "logging": {
            "level": "info"
        }
    });

    let incoming = json!({
        "database": {
            "port": 3306,
            "username": "admin"
        }
    });

    let result = merger.merge_sources(vec![(base, 1), (incoming, 2)]);

    // Replace strategy should completely replace the value
    assert_eq!(result["database"]["port"], 3306);
    assert_eq!(result["database"]["username"], "admin");
    assert!(result["database"].get("host").is_none());
    assert!(result.get("logging").is_none());
}

#[test]
fn test_append_merge_arrays() {
    let merger = ConfigMerger::new(MergeStrategy::Append);

    let base = json!({
        "plugins": ["auth", "logging"],
        "features": {
            "enabled": ["feature1"]
        }
    });

    let incoming = json!({
        "plugins": ["metrics", "tracing"],
        "features": {
            "enabled": ["feature2", "feature3"]
        }
    });

    let result = merger.merge_sources(vec![(base, 1), (incoming, 2)]);

    let plugins = result["plugins"].as_array().unwrap();
    assert_eq!(plugins.len(), 4);
    assert!(plugins.contains(&json!("auth")));
    assert!(plugins.contains(&json!("logging")));
    assert!(plugins.contains(&json!("metrics")));
    assert!(plugins.contains(&json!("tracing")));

    let features = result["features"]["enabled"].as_array().unwrap();
    assert_eq!(features.len(), 3);
}

#[test]
fn test_priority_ordering() {
    let merger = ConfigMerger::new(MergeStrategy::Deep);

    let low_priority = json!({
        "value": "low",
        "only_low": "yes"
    });

    let medium_priority = json!({
        "value": "medium",
        "only_medium": "yes"
    });

    let high_priority = json!({
        "value": "high",
        "only_high": "yes"
    });

    // Sources are sorted by priority before merging
    let result = merger.merge_sources(vec![
        (high_priority, 3),
        (low_priority, 1),
        (medium_priority, 2),
    ]);

    assert_eq!(result["value"], "high");
    assert_eq!(result["only_low"], "yes");
    assert_eq!(result["only_medium"], "yes");
    assert_eq!(result["only_high"], "yes");
}

#[test]
fn test_null_value_handling() {
    let merger = ConfigMerger::new(MergeStrategy::Deep);

    let base = json!({
        "field1": "value1",
        "field2": "value2"
    });

    let incoming = json!({
        "field1": null,
        "field3": "value3"
    });

    let result = merger.merge_sources(vec![(base, 1), (incoming, 2)]);

    // Null values should override
    assert_eq!(result["field1"], serde_json::Value::Null);
    assert_eq!(result["field2"], "value2");
    assert_eq!(result["field3"], "value3");
}
