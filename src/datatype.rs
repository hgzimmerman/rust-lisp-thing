use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use lang_result::*;
use Ast;


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Datatype {
    Number(i32),
    String(String),
    Array {
        value: Vec<Datatype>,
        type_: Box<TypeInfo>, // the type of data allowed in the array.
    },
    Bool(bool),
    None,
    Function {
        parameters: Box<Ast>,
        body: Box<Ast>,
        return_type: Box<TypeInfo>,
    },
    //Object { value: Vec<Datatype>, v_table: Vec<Ast>}
}

impl Sub for Datatype {
    type Output = LangResult;
    fn sub(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs - rhs)),
                    _ => Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Add for Datatype {
    type Output = LangResult;
    fn add(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs + rhs)),
                    Datatype::String(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the string to the number.
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::String(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the number to the string
                    }
                    Datatype::String(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the string to the string
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Mul for Datatype {
    type Output = LangResult;
    fn mul(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs * rhs)),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Div for Datatype {
    type Output = LangResult;
    fn div(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        if rhs == 0 {
                            return Err(LangError::DivideByZero);
                        }
                        return Ok(Datatype::Number(lhs / rhs));
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Rem for Datatype {
    type Output = LangResult;
    fn rem(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs % rhs)),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum TypeInfo {
    Number,
    String,
    Array,
    Bool,
    None,
    Function
}


impl From<Datatype> for TypeInfo {
    fn from(dt: Datatype) -> TypeInfo {
        use Datatype;
        match dt {
            Number => TypeInfo::Number,
            String => TypeInfo::String,
            Array => TypeInfo::Array,
            Bool => TypeInfo::Bool,
            Datatype::None => TypeInfo::None,
            Function => TypeInfo::Function
        }
    }
}


// Todo consider creating another enum that is just the type info that implements From<Datatype>
pub const NUMBER_TYPE: Datatype = Datatype::Number(0);
//pub const STRING_TYPE: Datatype = Datatype::String("".to_string()); // to_string isn't a constant function, therefore this is invalid
pub const BOOL_TYPE: Datatype = Datatype::Bool(false);