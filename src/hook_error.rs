use std::fmt;

#[derive(Debug)]
pub struct HookInstallationError {
    message: String,
}

impl HookInstallationError {
    pub fn new(message: &str) -> HookInstallationError {
        HookInstallationError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HookInstallationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
