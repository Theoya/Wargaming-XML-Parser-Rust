# Rust XML Parser

A powerful Rust library for parsing XML files, with specialized support for BattleScribe roster files (.rosz, .ros, .cat) and constraint validation. This project provides tools for decompressing, parsing, and validating XML-based game roster files commonly used in tabletop wargaming.

## 🎯 Features

- **XML Parsing**: Robust XML parsing with support for complex nested structures
- **File Decompression**: Decompress `.rosz` files (ZIP-based roster files)
- **Constraint Validation**: Validate XML constraints for selection counts, points limits, and more
- **Lexical Analysis**: Advanced token parsing and analysis
- **BattleScribe Support**: Specialized parsing for BattleScribe roster files
- **Error Handling**: Comprehensive error handling with detailed error messages

## 📁 Project Structure

```
Rust-XML-Parser/
├── Cargo.toml                 # Project configuration and dependencies
├── src/
│   ├── lib.rs                 # Library entry point
│   ├── main.rs                # CLI application entry point
│   ├── XmlParser.rs           # Main XML parsing logic
│   ├── models/                # Data structures and models
│   │   ├── mod.rs
│   │   ├── Constraint.rs      # Constraint data model
│   │   ├── ConstraintType.rs  # Constraint type definitions
│   │   ├── ValidationResult.rs # Validation result model
│   │   ├── XmlDocument.rs     # XML document model
│   │   ├── XmlElement.rs      # XML element model
│   │   └── XmlNode.rs         # XML node model
│   ├── Tools/                 # Core functionality modules
│   │   ├── mod.rs
│   │   ├── decompression.rs   # File decompression utilities
│   │   ├── lexical_analysis.rs # Token parsing and analysis
│   │   ├── parse_tokens.rs    # Token parsing logic
│   │   └── validator.rs       # Constraint validation engine
│   └── tests/                 # Test modules
│       ├── mod.rs
│       ├── decompression_tests.rs
│       ├── parse_tokens_tests.rs
│       ├── roster_validation_tests.rs
│       └── validator_tests.rs
├── examples/
│   └── validator_example.rs   # Comprehensive validation examples
├── docs/
│   └── validator_usage.md     # Detailed validator documentation
└── example-data/              # Sample files for testing
    ├── Orks.cat
    ├── Test-Chaos-Thousand Sons.cat
    ├── Test-sample-army.rosz
    ├── Tts game teams.ros
    └── Tts Ork game teams.rosz
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/xml-parser.git
cd xml-parser
```

2. Build the project:
```bash
cargo build
```

### Running the Application

#### CLI Application
```bash
# Run the main CLI application
cargo run

# This will:
# - Decompress example .rosz files
# - Parse XML content
# - Demonstrate constraint validation
```

#### Examples
```bash
# Run the comprehensive validator example
cargo run --example validator_example

# This demonstrates:
# - Basic constraint validation
# - Selection entry group validation
# - Field-specific validation
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test validator
cargo test decompression
cargo test parse_tokens

# Run tests with output
cargo test -- --nocapture
```

## 📖 Usage Examples

### 1. Decompressing .rosz Files

```rust
use xml_parser::Tools::decompression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decompress a .rosz file
    let files = decompression::decompress_rosz_file("example-data/Tts Ork game teams.rosz")?;
    
    println!("Decompressed {} files:", files.len());
    for file in &files {
        println!("  - {}", file.filename);
        // Process file.content as needed
    }
    
    Ok(())
}
```

### 2. Constraint Validation

```rust
use xml_parser::Tools::validator::ConstraintValidator;
use xml_parser::models::Constraint::Constraint;
use xml_parser::models::ConstraintType::ConstraintType;

fn main() {
    let mut validator = ConstraintValidator::new();
    
    // Add constraints
    let min_constraint = Constraint {
        constraint_type: ConstraintType::Min,
        value: 2,
        field: "selections".to_string(),
        scope: "parent".to_string(),
        shared: true,
        id: "test-id".to_string(),
        include_child_selections: Some(true),
        include_child_forces: None,
        percent_value: None,
    };
    
    validator.add_constraint(min_constraint);
    
    // Validate selections
    let results = validator.validate_selections(3);
    for result in results {
        if result.is_valid {
            println!("✓ {}", result.message);
        } else {
            println!("✗ {}", result.message);
        }
    }
}
```

### 3. XML Parsing and Validation

```rust
use xml_parser::Tools::validator::ConstraintValidator;
use xml_parser::models::XmlElement::XmlElement;

fn main() -> Result<(), String> {
    // Parse XML constraints
    let xml_string = r#"
        <constraints>
            <constraint type="min" value="2" field="selections" scope="parent" shared="true" id="c6ca-55be-a638-5f08"/>
            <constraint type="max" value="5" field="selections" scope="parent" shared="true" id="ad81-4838-8c1d-8c1c"/>
        </constraints>
    "#;
    
    let validator = ConstraintValidator::from_xml_string(xml_string)?;
    
    // Validate different selection counts
    let test_values = vec![1, 2, 3, 5, 6];
    for value in test_values {
        let results = validator.validate_selections(value);
        println!("Selection count {}: ", value);
        for result in results {
            let status = if result.is_valid { "✓" } else { "✗" };
            println!("  {} {}", status, result.message);
        }
    }
    
    Ok(())
}
```

## 🔧 Core Components

### 1. Decompression Module (`src/Tools/decompression.rs`)
- Decompresses `.rosz` files (ZIP-based roster files)

### 2. Validator Module (`src/Tools/validator.rs`)
- Validates XML constraints for selection counts, points limits, etc.
- Supports constraint types: min, max, equal, notEqual, atLeast, atMost
- Provides comprehensive validation results with detailed messages in the event something fails

### 3. Lexical Analysis (`src/Tools/lexical_analysis.rs`)
- Parses XML structures

### 4. Parse Tokens (`src/Tools/parse_tokens.rs`)
- Token-based XML parsing to follow up on lexical_analysis

## 📋 Constraint Types

The validator supports the following constraint types:

| Type | Description | Example |
|------|-------------|---------|
| `min` | Value must be ≥ specified value | `value="2"` |
| `max` | Value must be ≤ specified value | `value="5"` |
| `equal` | Value must be exactly equal | `value="3"` |
| `notEqual` | Value must not be equal | `value="0"` |
| `atLeast` | Alias for min | `value="1"` |
| `atMost` | Alias for max | `value="10"` |

## 🧪 Testing

The project includes comprehensive tests for all major components:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test validator_tests
cargo test decompression_tests
cargo test parse_tokens_tests
cargo test roster_validation_tests

# Run tests with verbose output
cargo test -- --nocapture
```

## 📚 Documentation

- **Validator Usage**: See `docs/validator_usage.md` for detailed validator documentation
- **Examples**: Check `examples/validator_example.rs` for comprehensive usage examples
- **API Documentation**: Generate with `cargo doc --open`


## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎮 Use Cases

This XML parser is particularly useful for:

- **Tabletop Wargaming**: Parse BattleScribe roster files.
- **Game Development**: Validate game rules and constraints
- **Data Analysis**: Extract and analyze roster data
- **Tool Development**: Build applications that work with roster files

## 🔍 Example Output

When running the validator example, you'll see output like:

```
=== XML Constraint Validator Example ===

1. Basic Constraint Validation:
  Selection count 1: 
    ✗ Selection count (1) is below minimum (2)
    → Invalid configuration
  Selection count 2: 
    ✓ Selection count (2) meets minimum requirement (2)
    ✓ Selection count (2) is within maximum limit (5)
    → Valid configuration
  Selection count 3: 
    ✓ Selection count (3) meets minimum requirement (2)
    ✓ Selection count (3) is within maximum limit (5)
    → Valid configuration
```

## 🛠️ Dependencies

- `zip`: File decompression (used for .rosz file handling)
- `anyhow`: Error handling (used throughout the codebase)