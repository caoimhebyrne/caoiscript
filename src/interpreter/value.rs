#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(u32),
    None,
}
