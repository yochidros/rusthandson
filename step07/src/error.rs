#[derive(Debug)]
pub struct StringError {
    pub msg: String,
}
impl StringError {
    pub fn new(msg: &str) -> StringError {
        StringError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for StringError {
    fn description(&self) -> &str {
        &self.msg
    }
}
