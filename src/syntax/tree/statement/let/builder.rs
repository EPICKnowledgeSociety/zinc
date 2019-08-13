//!
//! The require expression builder.
//!

use crate::lexical::Token;
use crate::syntax::Let;

#[derive(Default)]
pub struct Builder {
    expression: Option<Vec<Token>>,
}

impl Builder {
    pub fn set_expression(&mut self, value: Vec<Token>) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> Let {
        Let::new(self.expression.take().expect("Missing expression"))
    }
}
