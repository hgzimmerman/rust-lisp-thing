#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use parser::body::body;
use std::boxed::Box;
use parser::expressions::sexpr;

named!(pub if_expression<Ast>,
    do_parse!(
        ws!(tag!("if")) >>
        if_conditional: ws!(sexpr) >>
        if_body: ws!(body) >>
        else_body: opt!(
            complete!(
                // nest another do_parse to get the else keyword and its associated block
                do_parse!(
                    ws!(tag!("else")) >>
                    e: map!( // Map the body of the else statement into a Box so it can easily live in the Some()
                        ws!(body),
                        Box::new
                    ) >>
                    (e)
                )

            )
        ) >>
        (
        Ast::Conditional {
            condition: Box::new(if_conditional),
            true_expr: Box::new(if_body),
            false_expr: else_body
        })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;
    use s_expression::SExpression;

    #[test]
    fn parse_if_statement_test() {
        let input_string = "if true { true }";
        let (_, value) = match if_expression(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
        assert_eq!(
            Ast::Conditional {
                condition: Box::new(Ast::Literal(Datatype::Bool(true))),
                true_expr: Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                )),
                false_expr: None,
            },
            value
        )
    }


    #[test]
    fn parse_if_statement_with_expression_test() {
        let input_string = "if 1 == 1 { true }";
        let (_, value) = match if_expression(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
        assert_eq!(
            Ast::Conditional {
                condition: Box::new(Ast::SExpr(SExpression::Equals(
                    Box::new(Ast::Literal(Datatype::Number(1))),
                    Box::new(Ast::Literal(Datatype::Number(1))),
                ))),
                true_expr: Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                )),
                false_expr: None,
            },
            value
        )
    }
    #[test]
    fn parse_if_else_statement_test() {
        let input_string = "if true { true } else { true }";
        let (_, value) = match if_expression(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::Conditional {
                condition: Box::new(Ast::Literal(Datatype::Bool(true))),
                true_expr: Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                )),
                false_expr: Some(Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                ))),
            },
            value
        )
    }
}
