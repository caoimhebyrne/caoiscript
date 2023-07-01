#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(u32),
    None,
}

impl Value {
    pub fn add(&self, other: &Value) -> Value {
        match self {
            Value::Integer(_) => self.add_integers(other),
            _ => panic!("Unable to add {:?} and {:?}", self, other),
        }
    }

    fn add_integers(&self, other: &Value) -> Value {
        let Value::Integer(self_value) = self else {
            panic!("Expected {:?} to be Integer!", self);
        };

        let Value::Integer(other_value) = other else {
            panic!("Expected {:?} to be Integer!", other);
        };

        Value::Integer(self_value + other_value)
    }
}