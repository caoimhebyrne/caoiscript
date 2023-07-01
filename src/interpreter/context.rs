use std::collections::HashMap;
use std::fmt;

use super::value::Value;

#[derive(Debug)]
pub struct Context {
    name: String,
    variables: HashMap<String, Value>,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;

        for (name, value) in self.variables.clone() {
            writeln!(f, "  - {} = {:?}", name, value)?;
        }

        Ok(())
    }
}

impl Context {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.into(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }
}
