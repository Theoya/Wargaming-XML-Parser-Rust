use crate::models::Constraint::Constraint;
use crate::models::ConstraintType::ConstraintType;
use crate::models::ValidationResult::ValidationResult;
use crate::models::XmlElement::XmlElement;
use crate::models::XmlNode::XmlNode;
use crate::Tools::validator::ConstraintValidator;
use std::collections::HashMap;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_test_constraint_element(
    constraint_type: &str,
    value: &str,
    field: &str,
    scope: &str,
    shared: &str,
    id: &str,
) -> XmlElement {
    let mut attributes = HashMap::new();
    attributes.insert("type".to_string(), constraint_type.to_string());
    attributes.insert("value".to_string(), value.to_string());
    attributes.insert("field".to_string(), field.to_string());
    attributes.insert("scope".to_string(), scope.to_string());
    attributes.insert("shared".to_string(), shared.to_string());
    attributes.insert("id".to_string(), id.to_string());

    XmlElement {
        name: "constraint".to_string(),
        attributes,
        children: Vec::new(),
    }
}

fn create_test_constraints_element(constraints: Vec<XmlElement>) -> XmlElement {
    let children: Vec<XmlNode> = constraints.into_iter().map(XmlNode::Element).collect();

    XmlElement {
        name: "constraints".to_string(),
        attributes: HashMap::new(),
        children,
    }
}

fn create_test_constraint(
    constraint_type: ConstraintType,
    value: i32,
    field: &str,
    scope: &str,
    shared: bool,
    id: &str,
) -> Constraint {
    Constraint {
        constraint_type,
        value,
        field: field.to_string(),
        scope: scope.to_string(),
        shared,
        id: id.to_string(),
        include_child_selections: None,
        include_child_forces: None,
        percent_value: None,
    }
}

// ============================================================================
// POSITIVE TESTS - EXPECTED SUCCESS CASES
// ============================================================================

#[test]
fn test_parse_constraint_element_success() {
    let validator = ConstraintValidator::new();
    let constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_ok(),
        "Should successfully parse constraint element"
    );

    let constraint = result.unwrap();
    assert_eq!(constraint.constraint_type, ConstraintType::Min);
    assert_eq!(constraint.value, 2);
    assert_eq!(constraint.field, "selections");
    assert_eq!(constraint.scope, "parent");
    assert!(constraint.shared);
    assert_eq!(constraint.id, "test-id");
}

#[test]
fn test_parse_constraints_from_element_success() {
    let mut validator = ConstraintValidator::new();
    let constraint_element =
        create_test_constraint_element("max", "5", "selections", "parent", "true", "test-id");
    let constraints_element = create_test_constraints_element(vec![constraint_element]);

    let result = validator.parse_constraints_from_element(&constraints_element);
    assert!(
        result.is_ok(),
        "Should successfully parse constraints element"
    );
    assert_eq!(validator.constraint_count(), 1);
}

#[test]
fn test_validate_selections_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_selections(3);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value 3 should meet min constraint of 2"
    );
    assert!(results[0].message.contains("meets constraint"));
}

#[test]
fn test_validate_value_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Max,
        5,
        "test-field",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_value("test-field", 3);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value 3 should meet max constraint of 5"
    );
}

#[test]
fn test_validate_multiple_constraints_success() {
    let mut validator = ConstraintValidator::new();

    // Add min and max constraints for the same field
    let min_constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "min-id",
    );
    let max_constraint = create_test_constraint(
        ConstraintType::Max,
        5,
        "selections",
        "parent",
        true,
        "max-id",
    );

    validator.add_constraint(min_constraint);
    validator.add_constraint(max_constraint);

    let results = validator.validate_selections(3);
    assert_eq!(results.len(), 2);

    let all_valid = results.iter().all(|r| r.is_valid);
    assert!(
        all_valid,
        "Value 3 should meet both min(2) and max(5) constraints"
    );
}

#[test]
fn test_validate_equal_constraint_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Equal,
        3,
        "count",
        "parent",
        true,
        "equal-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 3);
    assert_eq!(results.len(), 1);
    assert!(results[0].is_valid, "Value 3 should equal constraint of 3");
}

#[test]
fn test_validate_not_equal_constraint_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::NotEqual,
        3,
        "count",
        "parent",
        true,
        "not-equal-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 4);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value 4 should not equal constraint of 3"
    );
}

#[test]
fn test_validate_at_least_constraint_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::AtLeast,
        2,
        "models",
        "unit",
        true,
        "at-least-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("models", 3);
    assert_eq!(results.len(), 1);
    assert!(results[0].is_valid, "Value 3 should be at least 2");
}

#[test]
fn test_validate_at_most_constraint_success() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::AtMost,
        5,
        "models",
        "unit",
        true,
        "at-most-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("models", 4);
    assert_eq!(results.len(), 1);
    assert!(results[0].is_valid, "Value 4 should be at most 5");
}

#[test]
fn test_get_constraints_for_field_success() {
    let mut validator = ConstraintValidator::new();

    let constraint1 = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "min-id",
    );
    let constraint2 = create_test_constraint(
        ConstraintType::Max,
        5,
        "selections",
        "parent",
        true,
        "max-id",
    );
    let constraint3 =
        create_test_constraint(ConstraintType::Min, 1, "models", "unit", true, "models-id");

    validator.add_constraint(constraint1);
    validator.add_constraint(constraint2);
    validator.add_constraint(constraint3);

    let selections_constraints = validator.get_constraints_for_field("selections");
    assert_eq!(selections_constraints.len(), 2);

    let models_constraints = validator.get_constraints_for_field("models");
    assert_eq!(models_constraints.len(), 1);
}

#[test]
fn test_get_constraints_by_type_success() {
    let mut validator = ConstraintValidator::new();

    let min_constraint1 = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "min-id-1",
    );
    let min_constraint2 =
        create_test_constraint(ConstraintType::Min, 1, "models", "unit", true, "min-id-2");
    let max_constraint = create_test_constraint(
        ConstraintType::Max,
        5,
        "selections",
        "parent",
        true,
        "max-id",
    );

    validator.add_constraint(min_constraint1);
    validator.add_constraint(min_constraint2);
    validator.add_constraint(max_constraint);

    let min_constraints = validator.get_constraints_by_type(&ConstraintType::Min);
    assert_eq!(min_constraints.len(), 2);

    let max_constraints = validator.get_constraints_by_type(&ConstraintType::Max);
    assert_eq!(max_constraints.len(), 1);
}

#[test]
fn test_from_selection_entry_group_constraints_success() {
    let constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "group-id");
    let constraints_element = create_test_constraints_element(vec![constraint_element]);

    let result = ConstraintValidator::from_selection_entry_group_constraints(&constraints_element);
    assert!(
        result.is_ok(),
        "Should successfully create validator from constraints"
    );

    let validator = result.unwrap();
    assert_eq!(validator.constraint_count(), 1);
}

// ============================================================================
// NEGATIVE TESTS - EXPECTED FAILURE CASES
// ============================================================================

#[test]
fn test_parse_constraint_element_missing_type() {
    let validator = ConstraintValidator::new();
    let mut constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");
    constraint_element.attributes.remove("type");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_err(),
        "Should fail when constraint type is missing"
    );
    assert!(result.unwrap_err().contains("Constraint type is required"));
}

#[test]
fn test_parse_constraint_element_invalid_type() {
    let validator = ConstraintValidator::new();
    let constraint_element =
        create_test_constraint_element("invalid", "2", "selections", "parent", "true", "test-id");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_err(),
        "Should fail when constraint type is invalid"
    );
    assert!(result.unwrap_err().contains("Unknown constraint type"));
}

#[test]
fn test_parse_constraint_element_missing_value() {
    let validator = ConstraintValidator::new();
    let mut constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");
    constraint_element.attributes.remove("value");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_err(),
        "Should fail when constraint value is missing"
    );
    assert!(result.unwrap_err().contains("Constraint value is required"));
}

#[test]
fn test_parse_constraint_element_invalid_value() {
    let validator = ConstraintValidator::new();
    let constraint_element =
        create_test_constraint_element("min", "invalid", "selections", "parent", "true", "test-id");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_err(),
        "Should fail when constraint value is not a valid integer"
    );
    assert!(result
        .unwrap_err()
        .contains("Constraint value must be a valid integer"));
}

#[test]
fn test_parse_constraint_element_missing_field() {
    let validator = ConstraintValidator::new();
    let mut constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");
    constraint_element.attributes.remove("field");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(
        result.is_err(),
        "Should fail when constraint field is missing"
    );
    assert!(result.unwrap_err().contains("Constraint field is required"));
}

#[test]
fn test_parse_constraint_element_missing_id() {
    let validator = ConstraintValidator::new();
    let mut constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");
    constraint_element.attributes.remove("id");

    let result = validator.parse_constraint_element(&constraint_element);
    assert!(result.is_err(), "Should fail when constraint id is missing");
    assert!(result.unwrap_err().contains("Constraint id is required"));
}

#[test]
fn test_parse_constraints_from_element_wrong_element_name() {
    let mut validator = ConstraintValidator::new();
    let constraint_element =
        create_test_constraint_element("min", "2", "selections", "parent", "true", "test-id");
    let mut constraints_element = create_test_constraints_element(vec![constraint_element]);
    constraints_element.name = "wrong".to_string();

    let result = validator.parse_constraints_from_element(&constraints_element);
    assert!(
        result.is_err(),
        "Should fail when element is not named 'constraints'"
    );
    assert!(result
        .unwrap_err()
        .contains("Element is not a constraints element"));
}

#[test]
fn test_validate_selections_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_selections(1);
    assert_eq!(results.len(), 1);
    assert!(
        !results[0].is_valid,
        "Value 1 should fail min constraint of 2"
    );
    assert!(results[0].message.contains("fails constraint"));
}

#[test]
fn test_validate_value_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Max,
        5,
        "test-field",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_value("test-field", 7);
    assert_eq!(results.len(), 1);
    assert!(
        !results[0].is_valid,
        "Value 7 should fail max constraint of 5"
    );
}

#[test]
fn test_validate_equal_constraint_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Equal,
        3,
        "count",
        "parent",
        true,
        "equal-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 4);
    assert_eq!(results.len(), 1);
    assert!(
        !results[0].is_valid,
        "Value 4 should not equal constraint of 3"
    );
}

#[test]
fn test_validate_not_equal_constraint_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::NotEqual,
        3,
        "count",
        "parent",
        true,
        "not-equal-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 3);
    assert_eq!(results.len(), 1);
    assert!(!results[0].is_valid, "Value 3 should equal constraint of 3");
}

#[test]
fn test_validate_at_least_constraint_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::AtLeast,
        2,
        "models",
        "unit",
        true,
        "at-least-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("models", 1);
    assert_eq!(results.len(), 1);
    assert!(!results[0].is_valid, "Value 1 should not be at least 2");
}

#[test]
fn test_validate_at_most_constraint_failure() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::AtMost,
        5,
        "models",
        "unit",
        true,
        "at-most-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("models", 6);
    assert_eq!(results.len(), 1);
    assert!(!results[0].is_valid, "Value 6 should not be at most 5");
}

#[test]
fn test_validate_nonexistent_field() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("nonexistent", 5);
    assert_eq!(
        results.len(),
        0,
        "Should return no results for nonexistent field"
    );
}

#[test]
fn test_validate_empty_validator() {
    let validator = ConstraintValidator::new();

    let results = validator.validate_selections(5);
    assert_eq!(results.len(), 0, "Empty validator should return no results");

    let results = validator.validate_field("any-field", 5);
    assert_eq!(results.len(), 0, "Empty validator should return no results");
}

#[test]
fn test_clear_constraints() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "test-id",
    );
    validator.add_constraint(constraint);

    assert_eq!(validator.constraint_count(), 1);

    validator.clear_constraints();
    assert_eq!(validator.constraint_count(), 0);

    let results = validator.validate_selections(5);
    assert_eq!(
        results.len(),
        0,
        "Cleared validator should return no results"
    );
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_validate_boundary_values() {
    let mut validator = ConstraintValidator::new();
    let min_constraint = create_test_constraint(
        ConstraintType::Min,
        2,
        "selections",
        "parent",
        true,
        "min-id",
    );
    let max_constraint = create_test_constraint(
        ConstraintType::Max,
        5,
        "selections",
        "parent",
        true,
        "max-id",
    );

    validator.add_constraint(min_constraint);
    validator.add_constraint(max_constraint);

    // Test boundary values
    let results_min = validator.validate_selections(2);
    assert_eq!(results_min.len(), 2);
    assert!(
        results_min.iter().all(|r| r.is_valid),
        "Value 2 should meet min constraint"
    );

    let results_max = validator.validate_selections(5);
    assert_eq!(results_max.len(), 2);
    assert!(
        results_max.iter().all(|r| r.is_valid),
        "Value 5 should meet max constraint"
    );
}

#[test]
fn test_validate_zero_values() {
    let mut validator = ConstraintValidator::new();
    let constraint =
        create_test_constraint(ConstraintType::Min, 0, "count", "parent", true, "zero-id");
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 0);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value 0 should meet min constraint of 0"
    );
}

#[test]
fn test_validate_negative_values() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Min,
        -5,
        "count",
        "parent",
        true,
        "negative-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", -3);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value -3 should meet min constraint of -5"
    );

    let results_fail = validator.validate_field("count", -7);
    assert_eq!(results_fail.len(), 1);
    assert!(
        !results_fail[0].is_valid,
        "Value -7 should fail min constraint of -5"
    );
}

#[test]
fn test_validate_large_values() {
    let mut validator = ConstraintValidator::new();
    let constraint = create_test_constraint(
        ConstraintType::Max,
        1000000,
        "count",
        "parent",
        true,
        "large-id",
    );
    validator.add_constraint(constraint);

    let results = validator.validate_field("count", 999999);
    assert_eq!(results.len(), 1);
    assert!(
        results[0].is_valid,
        "Value 999999 should meet max constraint of 1000000"
    );

    let results_fail = validator.validate_field("count", 1000001);
    assert_eq!(results_fail.len(), 1);
    assert!(
        !results_fail[0].is_valid,
        "Value 1000001 should fail max constraint of 1000000"
    );
}
