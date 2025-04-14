# jsonschema-for-toml

[![CI](https://github.com/foolishell/jsonschema-for-toml/actions/workflows/ci.yml/badge.svg)](https://github.com/foolishell/jsonschema-for-toml/actions/workflows/ci.yml)

A command-line tool for validating TOML files against JSON Schema.

This tool is inspired by [jsonschema-cli](https://github.com/Stranger6667/jsonschema/tree/master/crates/jsonschema-cli) and provides similar functionality, but specifically for TOML files.

## Features

- Validate TOML files against JSON Schema
- Support for multiple TOML files in a single run
- Detailed error reporting
- Clear validation results output

## Installation

```bash
cargo install jsonschema-for-toml
```

## Usage

```bash
jsonschema-for-toml <SCHEMA> -i <INSTANCE> [-i <INSTANCE>...]
```

### Arguments

- `<SCHEMA>`: Path to the JSON Schema file
- `-i, --instance`: Path to the TOML file(s) to validate (can be specified multiple times)

### Examples

Validate a single TOML file:

```bash
jsonschema-for-toml schema.json -i config.toml
```

Validate multiple TOML files:

```bash
jsonschema-for-toml schema.json -i config1.toml -i config2.toml
```

### Output

For each TOML file, the tool will output:

- `<filename> - VALID` if the file is valid
- `<filename> - INVALID. Errors:` followed by a list of errors if invalid

Example output:

```
config1.toml - VALID
config2.toml - INVALID. Errors:
1. "age" must be an integer
2. "email" must be a valid email address
```

## Exit Codes

- `0`: All TOML files are valid
- `1`: One or more TOML files are invalid, or there was an error

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Tests with Verbose Output

```bash
cargo test -- --nocapture
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [jsonschema-cli](https://github.com/Stranger6667/jsonschema/tree/master/crates/jsonschema-cli)
- Uses [jsonschema](https://crates.io/crates/jsonschema) for JSON Schema validation
- Uses [toml](https://crates.io/crates/toml) for TOML parsing
