use datatype::{Datatype, TypeInfo, BOOL_TYPE, NUMBER_TYPE};
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

named!(bool_false<bool>,
    do_parse!(
        tag!("false") >>
        (false)
    )
);
named!(bool_true<bool>,
    do_parse!(
        tag!("true") >>
        (true)
    )
);
named!(bool_literal<Ast>,
    do_parse!(
        boolean_value: alt!(bool_true | bool_false) >>
        (Ast::Literal {datatype: Datatype::Bool(boolean_value)})
    )
);

/// put all literal types here
named!(literal<Ast>,
    alt!(number_literal | string_literal | bool_literal)
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

named!(type_signature<Ast>,
   ws!(alt!(number_ts | string_ts | bool_ts))
);

named!(number_ts<Ast>,
    do_parse!(
        tag!("Number") >>
        (Ast::Type{datatype: TypeInfo::Number})
    )
);
named!(string_ts<Ast>,
    do_parse!(
        tag!("String") >>
        (Ast::Type{datatype: TypeInfo::String})
    )
);
named!(bool_ts<Ast>,
    do_parse!(
        tag!("Bool") >>
        (Ast::Type{datatype: TypeInfo::Bool})
    )
);

/// I want the function definition syntax to look like: fn fn_name(id: datatype, ...) -> return_type { expressions }

/// matches the signature:  identifier : expression|literal
/// Used for assigning identifiers to types
named!(function_parameter_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        type_info: type_signature >>
        (Ast::Expression{ operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier{ident: id}), expr2: Box::new(type_info) })
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

named!(function_return_type<TypeInfo>,
    do_parse!(
        ws!(tag!("->")) >>
        return_type: type_signature >>
        // Extract the datatype from the Ast::Type provided by the type_signature function
        (match return_type {
            Ast::Type {datatype} => datatype,
            _ => unreachable!() // this branch should never be encountered.
        })
    )
);

named!(function<Ast>,
    do_parse!(
        ws!(tag!("fn")) >>
        function_name: identifier >>
        arguments: delimited!(
            tag!("("),
            many0!(function_parameter_assignment),
            tag!(")")
        ) >>
        return_type: function_return_type >>
        body_expressions: delimited!(
            tag!("{"),
            many1!(binary_expr_parens), // todo: expand this to unary and assignments, others
            tag!("}")
        ) >>
        (Ast::Expression{
            operator: BinaryOperator::Assignment,
            expr1: Box::new(Ast::ValueIdentifier{ident: function_name}),
            expr2: Box::new(Ast::Literal {datatype: Datatype::Function {
                parameters: Box::new(Ast::VecExpression{expressions: arguments}),
                body: Box::new(Ast::VecExpression{expressions: body_expressions}),
                return_type: Box::new(return_type)
            } } )
        })
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
fn parse_bool_literal_test() {
    let (_, value) = match bool_literal(b"true") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal {datatype: Datatype::Bool(true)}, value)
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
    let input_string = "b : Number";
    let (_, value) = match function_parameter_assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()}), expr2: Box::new(Ast::Type {datatype: TypeInfo::Number}) }, value)
}