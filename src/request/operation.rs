use std::{fmt::Display, str::FromStr};

use super::errors::OperationParseError;

#[derive(Debug)]
pub enum Operation {
    Lock,
    Release,
}

impl Operation {
    fn repr(&self) -> &str {
        match self {
            Operation::Lock => "LOCK",
            Operation::Release => "RELEASE",
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LOCK" => Ok(Self::Lock),
            "RELEASE" => Ok(Self::Release),
            _ => Err(OperationParseError(format!(
                "Unable to parse '{}' as valid operation",
                s
            ))),
        }
    }
}
