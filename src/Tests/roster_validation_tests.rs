use crate::models::Constraint::Constraint;
use crate::models::ConstraintType::ConstraintType;
use crate::models::ValidationResult::ValidationResult;
use crate::models::XmlDocument::XmlDocument;
use crate::models::XmlElement::XmlElement;
use crate::models::XmlNode::XmlNode;
use crate::Tools::decompression::decompress_rosz_file;
use crate::Tools::lexical_analysis::tokenize;
use crate::Tools::parse_tokens::parse_tokens;
use crate::Tools::validator::ConstraintValidator;
use std::collections::HashMap;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Parse a catalog file and extract all constraints
fn parse_catalog_constraints(catalog_path: &str) -> Result<Vec<Constraint>, String> {
    // Read the catalog file
    let catalog_content = std::fs::read_to_string(catalog_path)
        .map_err(|e| format!("Failed to read catalog file: {}", e))?;

    // Tokenize and parse the catalog
    let tokens =
        tokenize(&catalog_content).map_err(|e| format!("Failed to tokenize catalog: {:?}", e))?;

    let document = parse_tokens(tokens).map_err(|e| format!("Failed to parse catalog: {:?}", e))?;

    let root = document
        .get_root_element()
        .ok_or("Catalog has no root element")?;

    // Extract all constraints from the catalog
    let mut all_constraints = Vec::new();
    extract_constraints_from_element(root, &mut all_constraints)?;

    Ok(all_constraints)
}

/// Recursively extract constraints from an XML element and its children
fn extract_constraints_from_element(
    element: &XmlElement,
    constraints: &mut Vec<Constraint>,
) -> Result<(), String> {
    // If this is a constraints element, parse its child constraints
    if element.name == "constraints" {
        for child in &element.children {
            if let XmlNode::Element(constraint_element) = child {
                if constraint_element.name == "constraint" {
                    let constraint = parse_constraint_from_element(constraint_element)?;
                    constraints.push(constraint);
                }
            }
        }
    }

    // Recursively process all child elements
    for child in &element.children {
        if let XmlNode::Element(child_element) = child {
            extract_constraints_from_element(child_element, constraints)?;
        }
    }

    Ok(())
}

/// Parse a single constraint element
fn parse_constraint_from_element(element: &XmlElement) -> Result<Constraint, String> {
    let constraint_type = match element.get_attribute("type") {
        Some(type_str) => match type_str.as_str() {
            "min" => ConstraintType::Min,
            "max" => ConstraintType::Max,
            "equal" => ConstraintType::Equal,
            "notEqual" => ConstraintType::NotEqual,
            "atLeast" => ConstraintType::AtLeast,
            "atMost" => ConstraintType::AtMost,
            _ => return Err(format!("Unknown constraint type: {}", type_str)),
        },
        None => return Err("Constraint type is required".to_string()),
    };

    let value = element
        .get_attribute("value")
        .ok_or("Constraint value is required")?
        .parse::<i32>()
        .map_err(|_| "Constraint value must be a valid integer")?;

    let field = element
        .get_attribute("field")
        .ok_or("Constraint field is required")?
        .clone();

    let scope = element
        .get_attribute("scope")
        .unwrap_or(&"parent".to_string())
        .clone();

    let shared = element
        .get_attribute("shared")
        .map(|s| s == "true")
        .unwrap_or(false);

    let id = element
        .get_attribute("id")
        .ok_or("Constraint id is required")?
        .clone();

    let include_child_selections = element
        .get_attribute("includeChildSelections")
        .map(|s| s == "true");

    let include_child_forces = element
        .get_attribute("includeChildForces")
        .map(|s| s == "true");

    let percent_value = element.get_attribute("percentValue").map(|s| s == "true");

    Ok(Constraint {
        constraint_type,
        value,
        field,
        scope,
        shared,
        id,
        include_child_selections,
        include_child_forces,
        percent_value,
    })
}

/// Parse a roster file and extract selection counts
fn parse_roster_selections(roster_path: &str) -> Result<HashMap<String, i32>, String> {
    // Read the roster file
    let roster_content = std::fs::read_to_string(roster_path)
        .map_err(|e| format!("Failed to read roster file: {}", e))?;

    // Tokenize and parse the roster
    let tokens =
        tokenize(&roster_content).map_err(|e| format!("Failed to tokenize roster: {:?}", e))?;

    let document = parse_tokens(tokens).map_err(|e| format!("Failed to parse roster: {:?}", e))?;

    let root = document
        .get_root_element()
        .ok_or("Roster has no root element")?;

    // Extract selection counts from the roster
    let mut selections = HashMap::new();
    extract_selections_from_element(root, &mut selections);

    Ok(selections)
}

/// Recursively extract selection counts from roster elements
fn extract_selections_from_element(element: &XmlElement, selections: &mut HashMap<String, i32>) {
    // If this is a selection, count it
    if element.name == "selection" {
        if let Some(entry_id) = element.get_attribute("entryId") {
            *selections.entry(entry_id.clone()).or_insert(0) += 1;
        }
    }

    // Recursively process all child elements
    for child in &element.children {
        if let XmlNode::Element(child_element) = child {
            extract_selections_from_element(child_element, selections);
        }
    }
}

/// Validate a roster against catalog constraints
fn validate_roster_against_catalog(
    roster_path: &str,
    catalog_path: &str,
) -> Result<Vec<ValidationResult>, String> {
    // Parse catalog constraints
    let catalog_constraints = parse_catalog_constraints(catalog_path)?;
    println!(
        "Parsed {} constraints from catalog",
        catalog_constraints.len()
    );

    // Parse roster selections
    let roster_selections = parse_roster_selections(roster_path)?;
    println!(
        "Parsed {} selection types from roster",
        roster_selections.len()
    );

    // Create validator with catalog constraints
    let mut validator = ConstraintValidator::new();
    for constraint in catalog_constraints {
        validator.add_constraint(constraint);
    }

    // Validate each selection against relevant constraints
    let mut all_results = Vec::new();

    // First, validate general "selections" constraints (which apply to all selections)
    let general_selection_count: i32 = roster_selections.values().sum();
    let general_results = validator.validate_field("selections", general_selection_count);
    all_results.extend(general_results);

    // Then validate specific selection constraints
    for (selection_id, count) in roster_selections {
        // Try to find constraints that match this specific selection ID
        let specific_results = validator.validate_field(&selection_id, count);
        all_results.extend(specific_results);

        // Also check if there are any constraints with field names that might be selection IDs
        // This handles cases where constraints reference specific selection IDs
        for constraint in &validator.get_constraints_for_field(&selection_id) {
            let is_valid = match constraint.constraint_type {
                ConstraintType::Min => count >= constraint.value,
                ConstraintType::Max => count <= constraint.value,
                ConstraintType::Equal => count == constraint.value,
                ConstraintType::NotEqual => count != constraint.value,
                ConstraintType::AtLeast => count >= constraint.value,
                ConstraintType::AtMost => count <= constraint.value,
            };

            let message = if is_valid {
                format!(
                    "Selection {} (count: {}) meets constraint {} {}",
                    selection_id,
                    count,
                    constraint.constraint_type.to_string(),
                    constraint.value
                )
            } else {
                format!(
                    "Selection {} (count: {}) fails constraint {} {}",
                    selection_id,
                    count,
                    constraint.constraint_type.to_string(),
                    constraint.value
                )
            };

            all_results.push(ValidationResult {
                is_valid,
                message,
                constraint: (*constraint).clone(),
            });
        }
    }

    Ok(all_results)
}

// ============================================================================
// POSITIVE TESTS - EXPECTED SUCCESS CASES
// ============================================================================

#[test]
fn test_parse_catalog_constraints_success() {
    let result = parse_catalog_constraints("example-data/Orks.cat");
    assert!(
        result.is_ok(),
        "Should successfully parse catalog constraints"
    );

    let constraints = result.unwrap();
    assert!(
        !constraints.is_empty(),
        "Should find constraints in the catalog"
    );

    println!("Found {} constraints in Orks.cat", constraints.len());

    // Verify we have some common constraint types
    let has_min = constraints
        .iter()
        .any(|c| matches!(c.constraint_type, ConstraintType::Min));
    let has_max = constraints
        .iter()
        .any(|c| matches!(c.constraint_type, ConstraintType::Max));

    assert!(has_min, "Should have min constraints");
    assert!(has_max, "Should have max constraints");
}

#[test]
fn test_parse_roster_selections_success() {
    let result = parse_roster_selections("example-data/Tts game teams.ros");
    assert!(
        result.is_ok(),
        "Should successfully parse roster selections"
    );

    let selections = result.unwrap();
    assert!(
        !selections.is_empty(),
        "Should find selections in the roster"
    );

    println!("Found {} selection types in roster", selections.len());

    // Print some selection counts for debugging
    for (id, count) in selections.iter().take(5) {
        println!("  Selection {}: {} instances", id, count);
    }
}

#[test]
fn test_validate_roster_against_catalog_success() {
    let result =
        validate_roster_against_catalog("example-data/Tts game teams.ros", "example-data/Orks.cat");

    assert!(
        result.is_ok(),
        "Should successfully validate roster against catalog"
    );

    let validation_results = result.unwrap();
    println!("Generated {} validation results", validation_results.len());

    // Count valid vs invalid results
    let valid_count = validation_results.iter().filter(|r| r.is_valid).count();
    let invalid_count = validation_results.len() - valid_count;

    println!(
        "Valid constraints: {}, Invalid constraints: {}",
        valid_count, invalid_count
    );

    // Print some validation results
    for result in validation_results.iter().take(10) {
        let status = if result.is_valid { "✓" } else { "✗" };
        println!("  {} {}", status, result.message);
    }
}

#[test]
fn test_decompress_and_validate_rosz_success() {
    // First decompress the .rosz file
    let decompress_result = decompress_rosz_file("example-data/Tts Ork game teams.rosz");
    assert!(
        decompress_result.is_ok(),
        "Should successfully decompress .rosz file"
    );

    let files = decompress_result.unwrap();
    assert_eq!(files.len(), 1, "Should decompress exactly one file");

    let roster_file = &files[0];
    assert!(
        roster_file.filename.ends_with(".ros"),
        "Should be a .ros file"
    );

    // Save the decompressed file temporarily
    let temp_path = "example-data/temp_roster.ros";
    std::fs::write(temp_path, &roster_file.content).expect("Failed to write temporary roster file");

    // Validate the decompressed roster against the catalog
    let validation_result = validate_roster_against_catalog(temp_path, "example-data/Orks.cat");
    assert!(
        validation_result.is_ok(),
        "Should successfully validate decompressed roster"
    );

    let results = validation_result.unwrap();
    println!("Decompressed roster validation: {} results", results.len());

    // Clean up temporary file
    let _ = std::fs::remove_file(temp_path);
}

// ============================================================================
// NEGATIVE TESTS - EXPECTED FAILURE CASES
// ============================================================================

#[test]
fn test_parse_catalog_constraints_nonexistent_file() {
    let result = parse_catalog_constraints("example-data/nonexistent.cat");
    assert!(
        result.is_err(),
        "Should fail when catalog file doesn't exist"
    );
    assert!(result.unwrap_err().contains("Failed to read catalog file"));
}

#[test]
fn test_parse_roster_selections_nonexistent_file() {
    let result = parse_roster_selections("example-data/nonexistent.ros");
    assert!(
        result.is_err(),
        "Should fail when roster file doesn't exist"
    );
    assert!(result.unwrap_err().contains("Failed to read roster file"));
}

#[test]
fn test_validate_roster_against_catalog_invalid_catalog() {
    // Create a temporary invalid catalog file
    let invalid_catalog = r#"<?xml version="1.0" encoding="UTF-8"?>
<catalogue>
    <constraints>
        <constraint type="invalid" value="not_a_number" field="test" id="test-id"/>
    </constraints>
</catalogue>"#;

    let temp_catalog_path = "example-data/temp_invalid.cat";
    std::fs::write(temp_catalog_path, invalid_catalog)
        .expect("Failed to write temporary invalid catalog");

    let result = parse_catalog_constraints(temp_catalog_path);
    assert!(result.is_err(), "Should fail with invalid constraint type");

    // Clean up
    let _ = std::fs::remove_file(temp_catalog_path);
}

#[test]
fn test_validate_roster_against_catalog_missing_required_attributes() {
    // Create a temporary catalog with missing required attributes
    let invalid_catalog = r#"<?xml version="1.0" encoding="UTF-8"?>
<catalogue>
    <constraints>
        <constraint value="5" field="test"/>
    </constraints>
</catalogue>"#;

    let temp_catalog_path = "example-data/temp_missing_attrs.cat";
    std::fs::write(temp_catalog_path, invalid_catalog)
        .expect("Failed to write temporary catalog with missing attributes");

    let result = parse_catalog_constraints(temp_catalog_path);
    assert!(
        result.is_err(),
        "Should fail with missing required attributes"
    );

    // Clean up
    let _ = std::fs::remove_file(temp_catalog_path);
}

#[test]
fn test_decompress_nonexistent_rosz_file() {
    let result = decompress_rosz_file("example-data/nonexistent.rosz");
    assert!(result.is_err(), "Should fail when .rosz file doesn't exist");
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_validate_empty_roster() {
    // Create a minimal empty roster
    let empty_roster = r#"<?xml version="1.0" encoding="UTF-8"?>
<roster>
    <forces>
        <force>
            <selections>
            </selections>
        </force>
    </forces>
</roster>"#;

    let temp_roster_path = "example-data/temp_empty.ros";
    std::fs::write(temp_roster_path, empty_roster).expect("Failed to write temporary empty roster");

    let result = validate_roster_against_catalog(temp_roster_path, "example-data/Orks.cat");
    assert!(result.is_ok(), "Should handle empty roster gracefully");

    let results = result.unwrap();
    println!("Empty roster validation: {} results", results.len());

    // Clean up
    let _ = std::fs::remove_file(temp_roster_path);
}

#[test]
fn test_validate_roster_with_no_constraints() {
    // Create a minimal catalog with no constraints
    let empty_catalog = r#"<?xml version="1.0" encoding="UTF-8"?>
<catalogue>
    <name>Empty Catalog</name>
</catalogue>"#;

    let temp_catalog_path = "example-data/temp_empty_catalog.cat";
    std::fs::write(temp_catalog_path, empty_catalog)
        .expect("Failed to write temporary empty catalog");

    let result =
        validate_roster_against_catalog("example-data/Tts game teams.ros", temp_catalog_path);
    assert!(result.is_ok(), "Should handle catalog with no constraints");

    let results = result.unwrap();
    assert_eq!(
        results.len(),
        0,
        "Should have no validation results with no constraints"
    );

    // Clean up
    let _ = std::fs::remove_file(temp_catalog_path);
}

#[test]
fn test_validate_large_catalog_performance() {
    // Test performance with the large Orks.cat file
    let start = std::time::Instant::now();

    let result = parse_catalog_constraints("example-data/Orks.cat");
    assert!(result.is_ok(), "Should handle large catalog efficiently");

    let constraints = result.unwrap();
    let parse_time = start.elapsed();

    println!(
        "Parsed {} constraints in {:?}",
        constraints.len(),
        parse_time
    );
    assert!(
        parse_time.as_millis() < 1000,
        "Should parse large catalog in under 1 second"
    );
}

#[test]
fn test_validate_roster_with_complex_constraints() {
    // Test validation with complex constraint scenarios
    let result =
        validate_roster_against_catalog("example-data/Tts game teams.ros", "example-data/Orks.cat");

    assert!(
        result.is_ok(),
        "Should handle complex constraint validation"
    );

    let results = result.unwrap();

    // Analyze constraint types
    let mut constraint_type_counts = HashMap::new();
    for result in &results {
        let constraint_type = format!("{:?}", result.constraint.constraint_type);
        *constraint_type_counts.entry(constraint_type).or_insert(0) += 1;
    }

    println!("Constraint type distribution:");
    for (constraint_type, count) in constraint_type_counts {
        println!("  {}: {}", constraint_type, count);
    }

    // Verify we have a reasonable number of validation results
    assert!(results.len() > 0, "Should have validation results");
    assert!(
        results.len() < 10000,
        "Should not have excessive validation results"
    );
}
