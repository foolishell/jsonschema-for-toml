use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn get_fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

#[test]
fn test_valid_toml() {
    let schema_path = get_fixture_path("schema.json");
    let instance_path = get_fixture_path("valid.toml");

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(schema_path)
        .arg("-i")
        .arg(instance_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("VALID"));
}

#[test]
fn test_invalid_type() {
    let schema_path = get_fixture_path("schema.json");
    let instance_path = get_fixture_path("invalid_type.toml");

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(schema_path)
        .arg("-i")
        .arg(instance_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("INVALID"))
        .stdout(predicate::str::contains("type"));
}

#[test]
fn test_missing_required_field() {
    let schema_path = get_fixture_path("schema.json");
    let instance_path = get_fixture_path("missing_required.toml");

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(schema_path)
        .arg("-i")
        .arg(instance_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("INVALID"))
        .stdout(predicate::str::contains("required"));
}

#[test]
fn test_invalid_email_format() {
    let schema_path = get_fixture_path("schema.json");
    let instance_path = get_fixture_path("invalid_email.toml");

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(schema_path)
        .arg("-i")
        .arg(instance_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("INVALID"))
        .stdout(predicate::str::contains(r#"not a "email""#));
}

#[test]
fn test_multiple_files() {
    let schema_path = get_fixture_path("schema.json");
    let valid_path = get_fixture_path("valid.toml");
    let invalid_path = get_fixture_path("invalid_type.toml");

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(schema_path)
        .arg("-i")
        .arg(valid_path)
        .arg("-i")
        .arg(invalid_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("valid.toml - VALID"))
        .stdout(predicate::str::contains("invalid_type.toml - INVALID"));
}

#[test]
fn test_nonexistent_schema() {
    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg("nonexistent.json")
        .arg("-i")
        .arg(get_fixture_path("valid.toml"))
        .assert()
        .failure();
}

#[test]
fn test_nonexistent_instance() {
    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(get_fixture_path("schema.json"))
        .arg("-i")
        .arg("nonexistent.toml")
        .assert()
        .failure();
}

#[test]
fn test_invalid_schema_json() {
    let mut invalid_schema = get_fixture_path("schema.json");
    invalid_schema.set_file_name("invalid_schema.json");
    std::fs::write(&invalid_schema, "{invalid json}").unwrap();

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(invalid_schema)
        .arg("-i")
        .arg(get_fixture_path("valid.toml"))
        .assert()
        .failure();
}

#[test]
fn test_invalid_toml_syntax() {
    let mut invalid_toml = get_fixture_path("valid.toml");
    invalid_toml.set_file_name("invalid_syntax.toml");
    std::fs::write(&invalid_toml, "name = 'unclosed string").unwrap();

    Command::cargo_bin("jsonschema-for-toml")
        .unwrap()
        .arg(get_fixture_path("schema.json"))
        .arg("-i")
        .arg(invalid_toml)
        .assert()
        .failure();
}
