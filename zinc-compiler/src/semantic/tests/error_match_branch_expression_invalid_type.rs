//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn test() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchExpressionInvalidType {
            location: Location::new(6, 14),
            expected: Type::boolean().to_string(),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            reference: Location::new(5, 14),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}