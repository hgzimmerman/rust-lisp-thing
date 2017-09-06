#[allow(unused_imports)]
use nom::*;
use ast::{Ast, BinaryOperator};
use parser::identifier::identifier;
use parser::body::body;
use parser::type_signature::type_signature;
use datatype::{Datatype, TypeInfo};
use parser::assignment::type_assignment;



named!(function_return_type<TypeInfo>,
    do_parse!(
        ws!(tag!("->")) >>
        return_type: type_signature >>
        // Extract the datatype from the Ast::Type provided by the type_signature function
        (match return_type {
            Ast::Type (datatype) => datatype,
            _ => unreachable!() // this branch should never be encountered. //TODO create an error
        })
    )
);

/// The function definition syntax should look like: fn fn_name(id: datatype, ...) -> return_type { expressions ...}
/// Where the id: datatype is optional
named!(pub function<Ast>,
    do_parse!(
        ws!(tag!("fn")) >>
        function_name: identifier >>
        arguments: delimited!(
            ws!(char!('(')),
            many0!(ws!(type_assignment)),
            ws!(char!(')'))
        ) >>
        return_type: function_return_type >>
        body_expressions: body >>
        (Ast::Expression{
            operator: BinaryOperator::Assignment,
            expr1: Box::new(function_name),
            expr2: Box::new(Ast::Literal (
                Datatype::Function {
                    parameters: Box::new(Ast::VecExpression{expressions: arguments}),
                    body: Box::new(body_expressions),
                    return_type: Box::new(return_type)
                }
            ) )
        })
    )
);







#[test]
fn parse_whole_function_number_input_returns_number_test() {
    let input_string = "fn test_function ( a : Number ) -> Number { ( a + 8 ) }";
    let (_, value) = match function(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let expected_fn: Ast = Ast::Expression {
        operator: BinaryOperator::Assignment,
        expr1: Box::new(Ast::ValueIdentifier("test_function".to_string())),
        expr2: Box::new(Ast::Literal(Datatype::Function {
            parameters: Box::new(Ast::VecExpression {
                expressions: vec![Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                        expr2: Box::new(Ast::Type ( TypeInfo::Number ))
                    }],
            }),
            body: Box::new(Ast::VecExpression {
                expressions: vec![
                        Ast::Expression {
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                            expr2: Box::new(Ast::Literal ( Datatype::Number(8))),
                        }],
            }),
            return_type: Box::new(TypeInfo::Number),
        })),
    };
    assert_eq!(expected_fn, value)
}
