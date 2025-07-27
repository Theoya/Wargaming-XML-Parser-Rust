use crate::models::Constraint::Constraint;
use crate::models::ConstraintType::ConstraintType;
use crate::models::ValidationResult::ValidationResult;
use crate::models::XmlElement::XmlElement;

pub struct ConstraintValidator {
    constraints: Vec<Constraint>,
}

impl ConstraintValidator {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn parse_constraints_from_element(&mut self, element: &XmlElement) -> Result<(), String> {
        if element.name != "constraints" {
            return Err("Element is not a constraints element".to_string());
        }

        for child in &element.children {
            if let crate::models::XmlNode::XmlNode::Element(constraint_element) = child {
                if constraint_element.name == "constraint" {
                    let constraint = self.parse_constraint_element(constraint_element)?;
                    self.add_constraint(constraint);
                }
            }
        }

        Ok(())
    }

    pub fn parse_constraint_element(&self, element: &XmlElement) -> Result<Constraint, String> {
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

    pub fn validate_value(&self, field_name: &str, value: i32) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        for constraint in &self.constraints {
            if constraint.field == field_name {
                let is_valid = match constraint.constraint_type {
                    ConstraintType::Min => value >= constraint.value,
                    ConstraintType::Max => value <= constraint.value,
                    ConstraintType::Equal => value == constraint.value,
                    ConstraintType::NotEqual => value != constraint.value,
                    ConstraintType::AtLeast => value >= constraint.value,
                    ConstraintType::AtMost => value <= constraint.value,
                };

                let message = if is_valid {
                    format!(
                        "Value {} meets constraint {} {}",
                        value,
                        constraint.constraint_type.to_string(),
                        constraint.value
                    )
                } else {
                    format!(
                        "Value {} fails constraint {} {}",
                        value,
                        constraint.constraint_type.to_string(),
                        constraint.value
                    )
                };

                results.push(ValidationResult {
                    is_valid,
                    message,
                    constraint: constraint.clone(),
                });
            }
        }

        results
    }

    pub fn validate_selections(&self, selection_count: i32) -> Vec<ValidationResult> {
        self.validate_value("selections", selection_count)
    }

    pub fn validate_field(&self, field_id: &str, value: i32) -> Vec<ValidationResult> {
        self.validate_value(field_id, value)
    }

    pub fn get_constraints_for_field(&self, field_name: &str) -> Vec<&Constraint> {
        self.constraints
            .iter()
            .filter(|c| c.field == field_name)
            .collect()
    }

    pub fn get_constraints_by_type(&self, constraint_type: &ConstraintType) -> Vec<&Constraint> {
        self.constraints
            .iter()
            .filter(|c| &c.constraint_type == constraint_type)
            .collect()
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    pub fn validate_xml_constraints_string(
        &self,
        xml_string: &str,
        field_name: &str,
        value: i32,
    ) -> Result<Vec<ValidationResult>, String> {
        // This is a simplified version - in a real implementation, you'd want to parse the XML properly
        // For now, we'll just validate against existing constraints
        Ok(self.validate_value(field_name, value))
    }

    pub fn from_xml_string(xml_string: &str) -> Result<Self, String> {
        // This would need to be implemented with proper XML parsing
        // For now, we'll return an empty validator
        // In a real implementation, you'd parse the XML and extract constraints
        Ok(Self::new())
    }

    pub fn from_selection_entry_group_constraints(
        constraints_element: &XmlElement,
    ) -> Result<Self, String> {
        let mut validator = Self::new();
        validator.parse_constraints_from_element(constraints_element)?;
        Ok(validator)
    }

    pub fn validate_selection_entry_group(
        &self,
        selection_count: i32,
        group_name: &str,
    ) -> Vec<ValidationResult> {
        println!(
            "Validating selection entry group: '{}' with {} selections",
            group_name, selection_count
        );
        self.validate_selections(selection_count)
    }
}
