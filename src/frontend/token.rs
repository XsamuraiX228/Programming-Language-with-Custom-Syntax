use crate::dialect::KeyWordType;
pub const VALID_OPERATORS: [char; 7] = ['+', '-', '*', '/', '^', '(', ')'];
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {   
    KeyWord(KeyWordType), // Let, Input, Print, If, Then, Random
    Ident(&'a str), // simple string
    Text(&'a str),
    Number(i64),  // i64 number
    Equal, // =
    DoubleEqual, // ==
    NonEqual, // !=
    Op(char), // ['+', '-', '*', '/', '^', '(', ')']
    Newline, // \n
    Mark(&'a str), // e.g :loop 
    Less,
    Greater,
}
