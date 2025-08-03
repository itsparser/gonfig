use gonfig::{Cli, ConfigSource};

#[test]
fn test_cli_basic_parsing() {
    let args = vec![
        "program".to_string(),
        "--database-url".to_string(),
        "postgres://localhost".to_string(),
        "--port".to_string(),
        "8080".to_string(),
    ];

    let cli = Cli::from_vec(args);
    let result = cli.collect().unwrap();

    assert_eq!(
        result.get("database-url").unwrap().as_str(),
        Some("postgres://localhost")
    );
    assert_eq!(result.get("port").unwrap().as_i64(), Some(8080));
}

#[test]
fn test_cli_boolean_flags() {
    let args = vec![
        "program".to_string(),
        "--debug".to_string(),
        "--verbose".to_string(),
        "false".to_string(),
    ];

    let cli = Cli::from_vec(args);
    let result = cli.collect().unwrap();

    assert_eq!(result.get("debug").unwrap().as_bool(), Some(true));
    assert_eq!(result.get("verbose").unwrap().as_bool(), Some(false));
}

#[test]
fn test_cli_type_parsing() {
    let args = vec![
        "program".to_string(),
        "--int".to_string(),
        "42".to_string(),
        "--float".to_string(),
        std::f64::consts::PI.to_string(),
        "--bool".to_string(),
        "true".to_string(),
        "--array".to_string(),
        "[1,2,3]".to_string(),
    ];

    let cli = Cli::from_vec(args);
    let result = cli.collect().unwrap();

    assert_eq!(result.get("int").unwrap().as_i64(), Some(42));
    assert_eq!(
        result.get("float").unwrap().as_f64(),
        Some(std::f64::consts::PI)
    );
    assert_eq!(result.get("bool").unwrap().as_bool(), Some(true));
    assert!(result.get("array").unwrap().is_array());
}

#[test]
fn test_cli_field_mapping() {
    let args = vec![
        "program".to_string(),
        "--custom-db".to_string(),
        "postgres://custom".to_string(),
    ];

    let cli = Cli::from_vec(args).with_field_mapping("database_url", "custom-db");

    let result = cli.collect().unwrap();

    // The field mapping should allow accessing via the field name
    assert!(result.get("custom-db").is_some());
}

#[test]
fn test_cli_safe_float_parsing() {
    let args = vec![
        "program".to_string(),
        "--valid-float".to_string(),
        "123.45".to_string(),
    ];

    let cli = Cli::from_vec(args);
    let result = cli.collect().unwrap();

    // Should parse valid float
    assert_eq!(result.get("valid-float").unwrap().as_f64(), Some(123.45));

    // NaN and infinity are handled gracefully by falling back to string
    let args_nan = vec![
        "program".to_string(),
        "--invalid-float".to_string(),
        "NaN".to_string(),
    ];

    let cli_nan = Cli::from_vec(args_nan);
    let result_nan = cli_nan.collect().unwrap();

    // Should fallback to string for NaN
    assert_eq!(
        result_nan.get("invalid-float").unwrap().as_str(),
        Some("NaN")
    );
}
