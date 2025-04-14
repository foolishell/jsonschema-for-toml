use anyhow::{Context, Result};
use clap::Parser;
use jsonschema::options;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON Schema file path
    schema: PathBuf,

    /// TOML instance file path (can be specified multiple times)
    #[arg(short = 'i', long = "instance", required = true)]
    instances: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read JSON Schema
    let schema_content = fs::read_to_string(&args.schema)
        .with_context(|| format!("Failed to read schema file: {:?}", args.schema))?;
    let schema: Value = serde_json::from_str(&schema_content)
        .with_context(|| format!("Failed to parse schema file: {:?}", args.schema))?;

    // Create validator
    let validator = match options().build(&schema) {
        Ok(v) => v,
        Err(error) => {
            println!("Schema is invalid. Error: {error}");
            std::process::exit(1);
        }
    };

    let mut has_errors = false;

    // Validate each TOML file
    for instance_path in args.instances {
        // Read and convert TOML to JSON
        let toml_content = fs::read_to_string(&instance_path)
            .with_context(|| format!("Failed to read TOML file: {:?}", instance_path))?;
        let toml_value: Value = toml::from_str(&toml_content)
            .with_context(|| format!("Failed to parse TOML file: {:?}", instance_path))?;

        let mut errors = validator.iter_errors(&toml_value);
        let filename = instance_path.to_string_lossy();

        if let Some(first) = errors.next() {
            has_errors = true;
            println!("{filename} - INVALID. Errors:");
            println!("1. {first}");
            for (i, error) in errors.enumerate() {
                println!("{}. {error}", i + 2);
            }
        } else {
            println!("{filename} - VALID");
        }
    }

    if has_errors {
        std::process::exit(1);
    }

    Ok(())
}
