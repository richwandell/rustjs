use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum JSOutput {
    Number {
        value: f64
    },
    String {
        value: String
    },
    Null
}

impl Display for JSOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JSOutput::Number {value} => {
                write!(f, "{}", value.to_string())
            }
            JSOutput::String {value} => {
                write!(f, "{}", value)
            }
            JSOutput::Null => {
                Result::Ok(())
            }
        }
    }
}

