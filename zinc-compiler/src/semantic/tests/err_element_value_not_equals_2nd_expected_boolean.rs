//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = boolean != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean(
            Type::new_integer_unsigned(8).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}