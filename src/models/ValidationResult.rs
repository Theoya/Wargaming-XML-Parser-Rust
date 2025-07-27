use crate::models::Constraint::Constraint;

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub message: String,
    pub constraint: Constraint,
}
