use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Contributor {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(range(min = 1, max = 5, message = "Seniority must be between 1 and 5"))]
    pub seniority: usize,
}

impl Contributor {
    pub fn new(name: String, seniority: usize) -> Contributor {
        return Contributor { name, seniority };
    }
}

impl Ord for Contributor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seniority.cmp(&other.seniority)
    }
}

impl PartialOrd for Contributor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Contributor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.seniority == other.seniority
    }
}

impl Eq for Contributor {}

impl Display for Contributor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{0}", self.name)
    }
}
