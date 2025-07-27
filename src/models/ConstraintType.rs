#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    Min,
    Max,
    Equal,
    NotEqual,
    AtLeast,
    AtMost,
}

impl std::fmt::Display for ConstraintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintType::Min => write!(f, "min"),
            ConstraintType::Max => write!(f, "max"),
            ConstraintType::Equal => write!(f, "equal"),
            ConstraintType::NotEqual => write!(f, "notEqual"),
            ConstraintType::AtLeast => write!(f, "atLeast"),
            ConstraintType::AtMost => write!(f, "atMost"),
        }
    }
}
