use std::collections::HashMap;
use xml_parser::models::Constraint::Constraint;
use xml_parser::models::ConstraintType::ConstraintType;
use xml_parser::models::XmlElement::XmlElement;
use xml_parser::models::XmlNode::XmlNode;
use xml_parser::Tools::validator::ConstraintValidator;

fn main() {
    println!("=== XML Constraint Validator Example ===\n");

    // Example 1: Basic constraint validation
    println!("1. Basic Constraint Validation:");
    basic_constraint_example();
    println!();

    // Example 2: Selection Entry Group validation
    println!("2. Selection Entry Group Validation:");
    selection_entry_group_example();
    println!();

    // Example 3: Field-specific validation
    println!("3. Field-Specific Validation:");
    field_validation_example();
    println!();
}

fn basic_constraint_example() {
    let mut validator = ConstraintValidator::new();

    // Add constraints similar to those found in the XML file
    let min_constraint = Constraint {
        constraint_type: ConstraintType::Min,
        value: 2,
        field: "selections".to_string(),
        scope: "parent".to_string(),
        shared: true,
        id: "c6ca-55be-a638-5f08".to_string(),
        include_child_selections: Some(true),
        include_child_forces: None,
        percent_value: None,
    };

    let max_constraint = Constraint {
        constraint_type: ConstraintType::Max,
        value: 5,
        field: "selections".to_string(),
        scope: "parent".to_string(),
        shared: true,
        id: "ad81-4838-8c1d-8c1c".to_string(),
        include_child_selections: Some(true),
        include_child_forces: None,
        percent_value: None,
    };

    validator.add_constraint(min_constraint);
    validator.add_constraint(max_constraint);

    // Test various selection counts
    let test_values = vec![1, 2, 3, 5, 6];

    for value in test_values {
        let results = validator.validate_selections(value);
        println!("  Selection count {}: ", value);

        let mut all_valid = true;
        for result in results {
            let status = if result.is_valid { "✓" } else { "✗" };
            println!("    {} {}", status, result.message);
            if !result.is_valid {
                all_valid = false;
            }
        }

        if all_valid {
            println!("    → Valid configuration");
        } else {
            println!("    → Invalid configuration");
        }
    }
}

fn selection_entry_group_example() {
    // Create a mock constraints element similar to the "2-5 Enlightened" group
    let constraints_element = create_mock_constraints_element();

    match ConstraintValidator::from_selection_entry_group_constraints(&constraints_element) {
        Ok(validator) => {
            println!("  Created validator from XML constraints");
            println!("  Constraints loaded: {}", validator.constraint_count());

            // Test the "2-5 Enlightened" group validation
            let test_configurations = vec![
                ("Too few", 1),
                ("Minimum", 2),
                ("Valid", 3),
                ("Maximum", 5),
                ("Too many", 6),
            ];

            for (description, count) in test_configurations {
                let results = validator.validate_selection_entry_group(count, "2-5 Enlightened");
                println!("  {} ({} selections):", description, count);

                let mut all_valid = true;
                for result in results {
                    let status = if result.is_valid { "✓" } else { "✗" };
                    println!("    {} {}", status, result.message);
                    if !result.is_valid {
                        all_valid = false;
                    }
                }

                if all_valid {
                    println!("    → Valid configuration");
                } else {
                    println!("    → Invalid configuration");
                }
            }
        }
        Err(e) => {
            println!("  Error creating validator: {}", e);
        }
    }
}

fn field_validation_example() {
    let mut validator = ConstraintValidator::new();

    // Add constraints for different fields (like points, models, etc.)
    let points_constraint = Constraint {
        constraint_type: ConstraintType::Max,
        value: 1000,
        field: "51b2-306e-1021-d207".to_string(), // Points field ID from XML
        scope: "force".to_string(),
        shared: true,
        id: "points-max".to_string(),
        include_child_selections: Some(true),
        include_child_forces: None,
        percent_value: None,
    };

    let models_constraint = Constraint {
        constraint_type: ConstraintType::Min,
        value: 1,
        field: "models".to_string(),
        scope: "unit".to_string(),
        shared: false,
        id: "models-min".to_string(),
        include_child_selections: None,
        include_child_forces: None,
        percent_value: None,
    };

    validator.add_constraint(points_constraint);
    validator.add_constraint(models_constraint);

    // Test different field validations
    println!("  Points validation:");
    let points_results = validator.validate_field("51b2-306e-1021-d207", 750);
    for result in points_results {
        let status = if result.is_valid { "✓" } else { "✗" };
        println!("    {} {}", status, result.message);
    }

    println!("  Models validation:");
    let models_results = validator.validate_field("models", 0);
    for result in models_results {
        let status = if result.is_valid { "✓" } else { "✗" };
        println!("    {} {}", status, result.message);
    }
}

fn create_mock_constraints_element() -> XmlElement {
    // Create constraint elements similar to the "2-5 Enlightened" group
    let mut min_attributes = HashMap::new();
    min_attributes.insert("type".to_string(), "min".to_string());
    min_attributes.insert("value".to_string(), "2".to_string());
    min_attributes.insert("field".to_string(), "selections".to_string());
    min_attributes.insert("scope".to_string(), "parent".to_string());
    min_attributes.insert("shared".to_string(), "true".to_string());
    min_attributes.insert("id".to_string(), "c6ca-55be-a638-5f08".to_string());

    let min_constraint = XmlElement {
        name: "constraint".to_string(),
        attributes: min_attributes,
        children: Vec::new(),
    };

    let mut max_attributes = HashMap::new();
    max_attributes.insert("type".to_string(), "max".to_string());
    max_attributes.insert("value".to_string(), "5".to_string());
    max_attributes.insert("field".to_string(), "selections".to_string());
    max_attributes.insert("scope".to_string(), "parent".to_string());
    max_attributes.insert("shared".to_string(), "true".to_string());
    max_attributes.insert("id".to_string(), "ad81-4838-8c1d-8c1c".to_string());

    let max_constraint = XmlElement {
        name: "constraint".to_string(),
        attributes: max_attributes,
        children: Vec::new(),
    };

    XmlElement {
        name: "constraints".to_string(),
        attributes: HashMap::new(),
        children: vec![
            XmlNode::Element(min_constraint),
            XmlNode::Element(max_constraint),
        ],
    }
}
