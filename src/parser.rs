use datatype::Datatype;
use ast::{Ast, BinaryOperator, UnaryOperator};
use nom::*;
use nom::IResult;
use std::str::FromStr;
use std::str;
use std::boxed::Box;



named!(plus<BinaryOperator>,
    do_parse!(
        tag!("+") >>
        (BinaryOperator::Plus)
    )
);
named!(minus<BinaryOperator>,
    do_parse!(
        tag!("-") >>
        (BinaryOperator::Minus)
    )
);

named!(multiply<BinaryOperator>,
     do_parse!(
        tag!("*") >>
        (BinaryOperator::Multiply)
    )
);
named!(divide<BinaryOperator>,
    do_parse!(
        tag!("/") >>
        (BinaryOperator::Divide)
    )
);
named!(modulo<BinaryOperator>,
    do_parse!(
        tag!("%") >>
        (BinaryOperator::Modulo)
    )
);

named!(binary_operator<BinaryOperator>,
    do_parse!(
        bin_op: ws!(alt!(plus | minus | multiply | divide | modulo)) >>
        (bin_op)
    )
);

named!(number<i32>,
    do_parse!(
        number: map_res!(
            map_res!(
                recognize!(
                    digit
                ),
                str::from_utf8
            ),
            FromStr::from_str
        ) >>
        (number)
    )
);
named!(number_literal<Ast>,
    do_parse!(
       num: ws!(number) >>
        (Ast::Literal {datatype: Datatype::Number(num)})
    )
);

named!(string<String>,
    do_parse!(
       str: map_res!(
            delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ),
            str::from_utf8
        ) >>
        (str.to_string())
    )
);

named!(string_literal<Ast>,
    do_parse!(
        str: ws!(string) >>
        (Ast::Literal {datatype: Datatype::String(str)})
    )
);

/// put all literal types here
named!(literal<Ast>,
    alt!(number_literal | string_literal)
);


named!(binary_expr<Ast>,
    do_parse!(
       op: binary_operator >>
       l1: literal >>
       l2: literal >>
       (Ast::Expression{ operator: BinaryOperator::Plus, expr1: Box::new(l1), expr2: Box::new(l2)})
    )
);
named!(binary_expr_parens<Ast>,
    delimited!(char!('('), binary_expr, char!(')'))
);

named!(identifier<String>,
    do_parse!(
        id: ws!(
            map_res!(
                alphanumeric, // todo: replace this with a combinator that accepts 0-9a-zA-Z as well as -,_
                str::from_utf8
            )
        ) >>
        (id.to_string())

    )
);


named!(assignment<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        value: ws!(literal) >> // I don't just want a literal, I could also use a bin expr, or a fn.
        (Ast::Expression{ operator: BinaryOperator::Assignment, expr1: Box::new(Ast::ValueIdentifier{ident: id}), expr2: Box::new(value) })
    )
);


/// I want the function definition syntax to look like: fn fn_name(id: datatype, ...) -> return_type { expressions }

/// matches the signature:  identifier : expression|literal
/// Used for assigning identifiers to types
named!(function_parameter_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        value: literal >>
        (Ast::Expression{ operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier{ident: id}), expr2: Box::new(value) })
    )
);

named!(function_body<Ast>,
    do_parse!(
        statements : delimited!(
            tag!("{"),
            many1!(ws!(binary_expr_parens)), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those.
            tag!("}")
        ) >>
        (Ast::VecExpression{expressions: statements})
    )
);


#[test]
fn parse_addition_test() {
    let (_, value) = match binary_expr(b"+ 3 4") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal {datatype: Datatype::Number(3)}), expr2:  Box::new(Ast::Literal {datatype: Datatype::Number(4)}) }, value);
}

#[test]
fn parse_addition_parens_test() {
    let (_, value) = match binary_expr_parens(b"(+ 3 4)") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal {datatype: Datatype::Number(3)}), expr2:  Box::new(Ast::Literal {datatype: Datatype::Number(4)}) }, value);
}

#[test]
fn parse_plus_test() {
    let (_, value) = match plus(b"+") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Plus, value)
}

#[test]
fn parse_operator_test() {
    let (_, value) = match binary_operator(b"%") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Modulo, value)
}

#[test]
fn parse_number_test() {
    let (_, value) = match number(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(42, value)
}

#[test]
fn parse_number_literal_test() {
    let (_, value) = match number_literal(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal {datatype: Datatype::Number(42)}, value)
}


#[test]
fn parse_string_test() {
    let input_string = "\"Hello World\"";
    let (_, value) = match string(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!("Hello World".to_string(), value)
}

#[test]
fn parse_string_literal_test() {
    let input_string = " \"Hello World\"  ";
    let (_, value) = match string_literal(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal {datatype: Datatype::String("Hello World".to_string())}, value)
}

#[test]
fn parse_string_and_number_addition_test() {
    let (_, value) = match binary_expr_parens(b"(+ 3 \"Hi\")") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal {datatype: Datatype::Number(3)}), expr2: Box::new(Ast::Literal {datatype: Datatype::String("Hi".to_string())}) }, value);
}

#[test]
fn parse_assignment_of_literal_test() {
    let input_string = "let b 8";
    let (_, value) = match assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Assignment, expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()}), expr2: Box::new(Ast::Literal {datatype: Datatype::Number(8)}) }, value)
}

#[test]
fn parse_function_parameter_assignment_of_literal_test() {
    let input_string = "b : 8";
    let (_, value) = match function_parameter_assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()}), expr2: Box::new(Ast::Literal {datatype: Datatype::Number(8)}) }, value)
}