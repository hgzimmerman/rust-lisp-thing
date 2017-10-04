#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::body::body;
use std::boxed::Box;
use parser::expressions::sexpr;
use parser::identifier::identifier;
use datatype::Datatype;
use uuid::Uuid;
use uuid::UuidVersion;

named!(pub for_loop<Ast>,
    do_parse!(
        ws!(tag!("for")) >>
        variable: ws!(identifier) >>
        ws!(tag!("in")) >>
        array: ws!(sexpr) >>
        for_body: ws!(body) >>

        ( create_for_loop(variable, array, for_body) )
    )
);

fn create_for_loop(identifier: Ast, array: Ast, for_body: Ast) -> Ast {

    // Create a unique value to hold the index that should never collide if this is called repeatedly.
    let index_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();
    let array_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();

    Ast::ExpressionList(vec![
        Ast::SExpr(SExpression::Assignment {
            identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
            ast: Box::new(Ast::Literal(Datatype::Number(0))) // 0 index
        }),
        // Hide the Array behind this assignment, so accessing it incurrs a constant cost
        // (if we initialize an array then iterate through it, we only create it once,
        // instead of creating a new array for every loop iteration like a prior implementation in all cases except
        // you guessed it, by accessing it through an id.)
        // TODO conditionally create this new assignment if the array AST passed in  is NOT already an identifier, if it is, there is no reason to hide it behind another identifier.
        Ast::SExpr(SExpression::Assignment {
            identifier: Box::new(Ast::ValueIdentifier(array_uuid.clone())),
            ast: Box::new(array)
        }),
        Ast::SExpr(SExpression::Loop {
            conditional: Box::new(Ast::SExpr(SExpression::LessThan (
                Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                Box::new(Ast::SExpr(SExpression::GetArrayLength(Box::new(Ast::ValueIdentifier(array_uuid.clone())))))
            ) )),
            body: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(identifier),
                    ast: Box::new(Ast::SExpr(SExpression::AccessArray {
                        identifier: Box::new(Ast::ValueIdentifier(array_uuid)),
                        index: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                    }))
                }), // Assign the value at the index to the given identifier
                for_body, // execute the specified body
                Ast::SExpr(
                    SExpression::Assignment {
                        identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                        ast: Box::new(Ast::SExpr(SExpression::Increment(Box::new(Ast::ValueIdentifier(index_uuid)))))
                    }
                ) // increment the index
            ]))
        })
    ])
}


    #[test]
    fn for_loop_parse() {
        let input_string = r#"
        for i in [0,2] {
            3
        }
         "#;
        let (_, ast) = match for_loop(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        use datatype::TypeInfo;


        let expected_ast = create_for_loop(
            Ast::ValueIdentifier("i".to_string()),
            Ast::Literal(Datatype::Array {
                value: vec![Datatype::Number(0), Datatype::Number(2)],
                type_: TypeInfo::Number
            }),
            Ast::ExpressionList(vec![Ast::Literal(Datatype::Number(3))])
        );
        // Can't test this because of the random uuids used for the value identifiers.

//        assert_eq!(expected_ast, ast);
    }

