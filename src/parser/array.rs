use ast::{Ast, BinaryOperator};
#[allow(unused_imports)]
use nom::*;
use std::boxed::Box;
use super::literal::number::number_literal;
use super::utilities::expression_or_literal_or_identifier;

named!(pub array_access<Ast>,
    do_parse!(
        array: expression_or_literal_or_identifier >>
        index: delimited!(
            ws!(char!('[')),
            number_literal,
            ws!(char!(']'))
        ) >>
        (Ast::Expression{
            operator: BinaryOperator::AccessArray,
            expr1: Box::new(array),
            expr2: Box::new(index)
        })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use datatype::{Datatype, TypeInfo};
    use nom::IResult;


    #[test]
    fn parse_array_access_test() {
        let input_string = r##"
        array_identifier[0]
        "##;
        let (_, value) = match array_access(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::Expression {
                operator: BinaryOperator::AccessArray,
                expr1: Box::new(Ast::ValueIdentifier("array_identifier".to_string())),
                expr2: Box::new(Ast::Literal(Datatype::Number(0))),
            },
            value
        )
    }

    #[test]
    fn parse_array_access_on_new_array_test() {
        let input_string = r##"
        [12, 13, 14][0]
        "##;
        let (_, value) = match array_access(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::Expression {
                operator: BinaryOperator::AccessArray,
                expr1: Box::new(Ast::Literal(Datatype::Array {
                    value: vec![
                        Datatype::Number(12),
                        Datatype::Number(13),
                        Datatype::Number(14),
                    ],
                    type_: TypeInfo::Number,
                })),
                expr2: Box::new(Ast::Literal(Datatype::Number(0))),
            },
            value
        )
    }
}
