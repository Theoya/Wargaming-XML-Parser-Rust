use crate::models::ConstraintType::ConstraintType;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub value: i32,
    pub field: String,
    pub scope: String,
    pub shared: bool,
    pub id: String,
    pub include_child_selections: Option<bool>,
    pub include_child_forces: Option<bool>,
    pub percent_value: Option<bool>,
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constraint {{ type: {}, value: {}, field: {}, scope: {}, id: {} }}",
            self.constraint_type, self.value, self.field, self.scope, self.id
        )
    }
}
