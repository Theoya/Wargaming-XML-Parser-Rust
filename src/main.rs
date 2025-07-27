use xml_parser::models::Constraint::Constraint;
use xml_parser::models::ConstraintType::ConstraintType;
use xml_parser::Tools::decompression;
use xml_parser::Tools::validator::ConstraintValidator;

fn main() {
    println!("XML Parser CLI");
    println!("Use this tool to parse XML files and compressed .rosz files");

    // Example usage
    match decompression::decompress_rosz_file("example-data/Tts Ork game teams.rosz") {
        Ok(files) => {
            println!("Successfully decompressed {} files:", files.len());
            for file in &files {
                println!("  - {}", file.filename);

                // Save the decompressed file to example-data directory
                let output_path = format!("example-data/{}", file.filename);
                if let Err(e) = std::fs::write(&output_path, &file.content) {
                    eprintln!("Error writing file {}: {:?}", output_path, e);
                } else {
                    println!("    Saved to: {}", output_path);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    // Example constraint validation
    println!("\n=== Constraint Validation Example ===");

    let mut validator = ConstraintValidator::new();

    // Add some example constraints similar to those in the XML file
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

    // Test validation with different values
    let test_values = vec![1, 2, 3, 5, 6];

    for value in test_values {
        let results = validator.validate_selections(value);
        println!("Validating selection count: {}", value);

        for result in results {
            println!("  - {}", result.message);
        }
        println!();
    }
}
