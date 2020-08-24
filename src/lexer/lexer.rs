use std::any::Any;
use crate::lexer::find_token::find_token;
use crate::lexer::string_iterator::StringIterator;
use crate::lexer::js_token::Tok;
use crate::ast::parser::Parser;
use crate::ast::ast::Expression;

pub enum LexError {
    Error { text: String },
    End
}

pub struct Lexer {
    pub(crate) tokens: Vec<Tok>
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new()
        }
    }

    pub fn parse(&mut self, mut parser: Parser, file: String) -> Vec<Expression> {
        let mut it = StringIterator::new(file.chars());
        parser.add_token(Tok::StartProgram);

        loop {
            let token = find_token(&mut it);

            match token {
                Ok(tokens) => {
                    for token in tokens {
                        parser.add_token(token);
                    }
                }
                Err(e) => {
                    match e {
                        LexError::Error { text } => {
                            self.test();
                            println!("{:?}", text);
                        },
                        LexError::End => {
                            break
                        }
                    }
                    break
                }
            }
        }
        return parser.ast_tree;
    }

    fn copy(&mut self) -> Vec<Tok> {
        let mut tokens = vec![];
        loop {
            let ex = self.tokens.pop();
            match ex {
                Some(token) => {
                    tokens.push(token);
                }
                None => {
                    break
                }
            }
        }
        tokens.reverse();
        return tokens;
    }

    pub fn lex(&mut self, file: String) -> Vec<Tok> {
        let mut it = StringIterator::new(file.chars());
        self.add_token(Tok::StartProgram);

        loop {
            let token = find_token(&mut it);

            match token {
                Ok(tokens) => {
                    for token in tokens {
                        self.add_token(token);
                    }
                }
                Err(e) => {
                    match e {
                        LexError::Error { text } => {
                            self.test();
                            println!("{:?}", text);
                        },
                        LexError::End => {
                            return self.copy();
                        }
                    }
                    break
                }
            }
        }
        return self.copy();
    }

    pub(crate) fn add_token(&mut self, token: Tok) {
        self.tokens.push(token);
    }

    pub fn test(&self) {
        for token in self.tokens.iter() {
            match token {
                Tok::Name {name} => println!("Name: {}", name),
                Tok::Await => println!("{}", "Await"),
                Tok::Break => println!("{}", "Break"),
                Tok::Case => println!("{}", "Case"),
                Tok::Catch => println!("{}", "Catch"),
                Tok::Class => println!("{}", "Class"),
                Tok::Const => println!("{}", "Const"),
                Tok::Continue => println!("{}", "Continue"),
                Tok::Debugger => println!("{}", "Debugger"),
                Tok::Default => println!("{}", "Default"),
                Tok::Delete => println!("{}", "Delete"),
                Tok::Do => println!("{}", "Do"),
                Tok::Else => println!("{}", "Else"),
                Tok::Enum => println!("{}", "Enum"),
                Tok::Export => println!("{}", "Export"),
                Tok::Extends => println!("{}", "Extends"),
                Tok::False => println!("{}", "False"),
                Tok::Finally => println!("{}", "Finally"),
                Tok::For => println!("{}", "For"),
                Tok::Function => println!("{}", "Function"),
                Tok::AnonFunction => println!("{}", "AnonFunction"),
                Tok::If => println!("{}", "If"),
                Tok::Implements => println!("{}", "Implements"),
                Tok::Import => println!("{}", "Import"),
                Tok::In => println!("{}", "In"),
                Tok::InstanceOf => println!("{}", "InstanceOf"),
                Tok::Interface => println!("{}", "Interface"),
                Tok::Let => println!("{}", "Let"),
                Tok::New => println!("{}", "New"),
                Tok::Null => println!("{}", "Null"),
                Tok::Package => println!("{}", "Package"),
                Tok::Private => println!("{}", "Private"),
                Tok::Protected => println!("{}", "Protected"),
                Tok::Public => println!("{}", "Public"),
                Tok::Return => println!("{}", "Return"),
                Tok::Super => println!("{}", "Super"),
                Tok::Switch => println!("{}", "Switch"),
                Tok::Static => println!("{}", "Static"),
                Tok::This => println!("{}", "This"),
                Tok::Throw => println!("{}", "Throw"),
                Tok::Try => println!("{}", "Try"),
                Tok::True => println!("{}", "True"),
                Tok::TypeOf => println!("{}", "TypeOf"),
                Tok::Var => println!("{}", "Var"),
                Tok::Void => println!("{}", "Void"),
                Tok::While => println!("{}", "While"),
                Tok::With => println!("{}", "With"),
                Tok::Yield => println!("{}", "Yield"),
                Tok::Float { value } => println!("Float: {}", value),
                Tok::String { value } => println!("String: {}", value),
                Tok::StartProgram => println!("{}", "StartProgram"),
                Tok::StartStatement => println!("{}", "StartStatement"),
                Tok::StartExpression => println!("{}", "StartExpression"),
                Tok::EndOfFile => println!("{}", "EndOfFile"),
                Tok::Lpar => println!("{}", "Lpar"),
                Tok::Rpar => println!("{}", "Rpar"),
                Tok::Lsqb => println!("{}", "Lsqb"),
                Tok::Rsqb => println!("{}", "Rsqb"),
                Tok::Colon => println!("{}", "Colon"),
                Tok::Comma => println!("{}", "Comma"),
                Tok::Semi => println!("{}", "Semi"),
                Tok::Plus => println!("{}", "Plus"),
                Tok::Minus => println!("{}", "Minus"),
                Tok::Star => println!("{}", "Star"),
                Tok::Slash => println!("{}", "Slash"),
                Tok::Bslash => println!("{}", "Bslash"),
                Tok::Vbar => println!("{}", "Vbar"),
                Tok::Amper => println!("{}", "Amper"),
                Tok::Less => println!("{}", "Less"),
                Tok::Greater => println!("{}", "Greater"),
                Tok::Equal => println!("{}", "Equal"),
                Tok::Dot => println!("{}", "Dot"),
                Tok::Percent => println!("{}", "Percent"),
                Tok::Lbrace => println!("{}", "Lbrace"),
                Tok::Rbrace => println!("{}", "Rbrace"),
                Tok::EqEqual => println!("{}", "EqEqual"),
                Tok::EqEqEual => println!("{}", "EqEqEual"),
                Tok::NotEqual => println!("{}", "NotEqual"),
                Tok::NotDoubleEqual => println!("{}", "NotDoubleEqual"),
                Tok::LessEqual => println!("{}", "LessEqual"),
                Tok::GreaterEqual => println!("{}", "GreaterEqual"),
                Tok::LeftShift => println!("{}", "LeftShift"),
                Tok::RightShift => println!("{}", "RightShift"),
                Tok::PlusEqual => println!("{}", "PlusEqual"),
                Tok::MinusEqual => println!("{}", "MinusEqual"),
                Tok::StarEqual => println!("{}", "StarEqual"),
                Tok::RsingleArrow => println!("{}", "RsingleArrow"),
                Tok::RdoubleArrow => println!("{}", "RdoubleArrow"),
                Tok::LeftShiftEqual => println!("{}", "LeftShiftEqual"),
                Tok::RightShiftEqual => println!("{}", "RightShiftEqual"),
                Tok::RightShiftUnsigned => println!("{}", "RigthShiftUnsigned"),
                Tok::RightShiftUnsignedEqual => println!("{}", "RightShiftUnsignedEqual"),
                Tok::EndOfLine => println!("{}", "EndOfLine")
            }
        }
    }
}