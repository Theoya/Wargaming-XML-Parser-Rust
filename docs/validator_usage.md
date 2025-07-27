# XML Constraint Validator

The XML Constraint Validator is a tool for validating constraints found in XML files, specifically designed for parsing and validating constraints in XML documents like those found in the example data.

## Overview

The validator can parse and validate constraints that look like this:

```xml
<constraints>
  <constraint type="min" value="2" field="selections" scope="parent" shared="true" id="c6ca-55be-a638-5f08"/>
  <constraint type="max" value="5" field="selections" scope="parent" shared="true" id="ad81-4838-8c1d-8c1c"/>
</constraints>
```

## Constraint Types

The validator supports the following constraint types:

- `min` - Value must be greater than or equal to the specified value
- `max` - Value must be less than or equal to the specified value
- `equal` - Value must be exactly equal to the specified value
- `notEqual` - Value must not be equal to the specified value
- `atLeast` - Value must be greater than or equal to the specified value (alias for min)
- `atMost` - Value must be less than or equal to the specified value (alias for max)

## Basic Usage

### Creating a Validator

```rust
use xml_parser::Tools::validator::ConstraintValidator;
use xml_parser::models::Constraint::Constraint;
use xml_parser::models::ConstraintType::ConstraintType;

let mut validator = ConstraintValidator::new();
```

### Adding Constraints Manually

```rust
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
```

### Parsing Constraints from XML Elements

```rust
use xml_parser::models::XmlElement::XmlElement;

// Assuming you have an XmlElement representing a <constraints> element
let constraints_element: XmlElement = /* your XML element */;
validator.parse_constraints_from_element(&constraints_element)?;
```

### Validating Values

```rust
// Validate selection counts
let results = validator.validate_selections(3);

// Validate any field
let results = validator.validate_field("selections", 3);

// Validate specific field by ID
let results = validator.validate_field("51b2-306e-1021-d207", 750);
```

### Working with Validation Results

```rust
for result in results {
    if result.is_valid {
        println!("✓ {}", result.message);
    } else {
        println!("✗ {}", result.message);
    }
}
```

## Advanced Usage

### Creating Validator from Selection Entry Group

```rust
// Create validator from a selectionEntryGroup's constraints
let validator = ConstraintValidator::from_selection_entry_group_constraints(&constraints_element)?;

// Validate a selection entry group
let results = validator.validate_selection_entry_group(3, "2-5 Enlightened");
```

### Filtering Constraints

```rust
// Get all constraints for a specific field
let field_constraints = validator.get_constraints_for_field("selections");

// Get all constraints of a specific type
let min_constraints = validator.get_constraints_by_type(&ConstraintType::Min);
```

## Example Scenarios

### 1. Validating Selection Entry Groups

This is useful for validating that the number of selections in a group meets the constraints:

```rust
// Example: "2-5 Enlightened" group
let mut validator = ConstraintValidator::new();

// Add min and max constraints
validator.add_constraint(Constraint {
    constraint_type: ConstraintType::Min,
    value: 2,
    field: "selections".to_string(),
    // ... other fields
});

validator.add_constraint(Constraint {
    constraint_type: ConstraintType::Max,
    value: 5,
    field: "selections".to_string(),
    // ... other fields
});

// Validate different selection counts
let test_values = vec![1, 2, 3, 5, 6];
for value in test_values {
    let results = validator.validate_selections(value);
    // Process results...
}
```

### 2. Validating Points Limits

For validating army point totals:

```rust
let points_constraint = Constraint {
    constraint_type: ConstraintType::Max,
    value: 1000,
    field: "51b2-306e-1021-d207".to_string(), // Points field ID
    scope: "force".to_string(),
    // ... other fields
};

validator.add_constraint(points_constraint);
let results = validator.validate_field("51b2-306e-1021-d207", 750);
```

### 3. Validating Model Counts

For ensuring minimum model counts in units:

```rust
let models_constraint = Constraint {
    constraint_type: ConstraintType::Min,
    value: 1,
    field: "models".to_string(),
    scope: "unit".to_string(),
    // ... other fields
};

validator.add_constraint(models_constraint);
let results = validator.validate_field("models", 0);
```

## Running Examples

To see the validator in action, run the example:

```bash
cargo run --example validator_example
```

Or run the main application which includes a basic validation example:

```bash
cargo run
```

## Testing

Run the validator tests:

```bash
cargo test validator
```

## Integration with XML Parser

The validator is designed to work seamlessly with the existing XML parser infrastructure. You can:

1. Parse XML files using the existing parser
2. Extract constraint elements from the parsed XML
3. Create validators from those elements
4. Validate values against the parsed constraints

This allows for a complete workflow from XML parsing to constraint validation. 