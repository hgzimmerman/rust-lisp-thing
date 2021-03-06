#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use parser::assignment::{type_assignment, struct_value_assignment, declaration};
use parser::control_flow::control_flow;
use parser::expressions::sexpr;

#[cfg(not(feature = "polite"))]
named!(pub body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(alt_complete!(sexpr| control_flow | declaration ))), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(char!('}'))
        ) >>
        (Ast::ExpressionList( statements ))
    )
);

// easter egg
#[cfg(feature = "polite")]
named!(pub body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(alt!(tag!("please") | tag!("{"))),
            many0!(ws!(alt_complete!( sexpr | control_flow | declaration ))), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(alt!(tag!("thankyou") | tag!("}")))
        ) >>

        (Ast::ExpressionList( statements ))
    )
);


///Body that only accepts assignments in the form: a : 4
named!(pub type_assignment_body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(type_assignment)),
            ws!(char!('}'))
        ) >>
        (Ast::ExpressionList( statements ))
    )
);

///Body that only accepts assignments in the form: a : <TYPE_NAME>
/// Used for creating a struct's type.
named!(pub struct_init_body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(struct_value_assignment)),
            ws!(char!('}'))
        ) >>
        (Ast::ExpressionList( statements ))
    )
);



#[test]
fn parse_body_nocheck_test() {
    let input_string = "{ a + 8 }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}

#[test]
fn parse_simple_body_test() {
    let input_string = "{ true }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}


#[test]
fn parse_simple_body_assignment_test() {
    let input_string = "{ let a := 8 }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}
