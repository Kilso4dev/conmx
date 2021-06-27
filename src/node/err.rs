use std::error::Error;

use std::fmt;

#[derive(Debug)]
pub struct NodeCreationErr(String);

impl fmt::Display for NodeCreationErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while creating Node: \"{}\"", self.0)
    }
}

impl Error for NodeCreationErr {
}


#[derive(Debug)]
pub struct NodeExecutionErr {
    driver: String,
    cause: String,
}

impl NodeExecutionErr {
    pub fn new(driver: String, cause: String) -> Self {
        Self {driver, cause}
    }
}

impl fmt::Display for NodeExecutionErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while execting driver function \"{}\", cause: {}", self.driver, self.cause)
    }
}

impl Error for NodeExecutionErr {
}
