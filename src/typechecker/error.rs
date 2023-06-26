use crate::location::Location;
use crate::typechecker::types::Type;

#[derive(Debug, Clone)]
pub struct TypecheckerError {
    pub location: Location,
    pub message: String,
}

impl TypecheckerError {
    pub fn mismatched_types(left: &Type, right: &Type, location: &Location) -> Self {
        Self {
            location: location.clone(),
            message: format!("Mismatched types: {} and {}", left.to_string(), right.to_string()),
        }
    }

    pub fn invalid_type(type_identifier: &str, location: &Location) -> Self {
        Self {
            location: location.clone(),
            message: format!("Invalid type: {}", type_identifier),
        }
    }
}

impl<T> From<TypecheckerError> for Result<T, TypecheckerError> {
    fn from(value: TypecheckerError) -> Self {
        Err(value)
    }
}

