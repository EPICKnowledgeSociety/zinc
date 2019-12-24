//!
//! The structure expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::StructureExpression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    path_expression: Option<Expression>,
    fields: Vec<(Identifier, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_path_expression(&mut self, value: Expression) {
        self.path_expression = Some(value);
    }

    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    pub fn set_field_expression(&mut self, value: Expression) {
        self.fields
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "field identifier"
                )
            })
            .1 = Some(value);
    }

    pub fn finish(mut self) -> StructureExpression {
        StructureExpression::new(
            self.location.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.path_expression.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "path expression"
                )
            }),
            self.fields
                .into_iter()
                .map(|(identifier, expression)| {
                    (
                        identifier,
                        expression.unwrap_or_else(|| {
                            panic!(
                                "{}{}",
                                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                                "field expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(Identifier, Expression)>>(),
        )
    }
}