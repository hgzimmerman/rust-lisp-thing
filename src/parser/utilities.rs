#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::literal::literal;
use parser::identifier::identifier;
use parser::structure::create_struct_instance;
use parser::expressions::{sexpr_parens, unary_operator_and_operand};


/// Any token that cannot directly recurse into itself (ie contain an expression as its first token)
/// nor contains a keyword.
///
/// This is used in the sexpr parser, as anything that could parse an expression could blow up the
/// stack, and that parser isn't interested in evaluating assignments, function definitions, etc...
named!(pub no_keyword_token_group <Ast>,
    alt_complete!(
        literal |
        identifier |
        create_struct_instance |
        sexpr_parens |
        unary_operator_and_operand // allows the s-expr to grap negated parameters
    )
);
