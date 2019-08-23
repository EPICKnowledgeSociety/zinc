//!
//! The literal lexeme.
//!

mod boolean;
mod integer;

pub use self::boolean::Boolean;
pub use self::integer::Integer;

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Literal {
    Boolean(Boolean),
    Integer(Integer),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}
