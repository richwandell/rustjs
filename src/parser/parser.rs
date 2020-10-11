use crate::lexer::js_token::Tok;
use crate::parser::symbols::{Expression, JSItem, Statement};
use crate::parser::find::assignment::{find_end_of_assignment};
use crate::parser::find::matching::{find_matching_brace};
use crate::parser::find::expression::find_end_of_expression;
use crate::parser::create::function::{create_function, create_arrow_function, create_function_assignment, create_function_expression};
use crate::parser::create::expression::{create_expression, create_assignment_expression};
use crate::parser::find::for_statement::find_end_of_for;
use crate::parser::create::for_statement::create_for_statement;
use crate::parser::find::function::find_end_of_function;
use crate::parser::create::block_statement::create_object_expression;

pub(crate) struct Parser {
    pub ast_tree: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SyntaxError {
    UnexpectedToken {
        tok: Tok
    },
    UnexpectedIdentifier {
        name: String
    },
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AssignmentType {
    ArrowFunction {
        end: usize
    },
    FunctionAssignment {
        end: usize
    },
    Unknown {
        end: usize
    },
    Expression {
        end: usize
    },
    ObjectExpression {
        end: usize
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FunctionType {
    FunctionExpression {
        end: usize
    },
    FunctionDeclaration {
        end: usize
    }
}



impl Parser {
    pub fn parse(&mut self, mut tokens: Vec<Tok>) -> Vec<JSItem> {
        if tokens.get(0).unwrap().eq(&Tok::EndOfLine) {
            tokens.remove(0);
        }

        if tokens.len() == 1 {
            let token = tokens.get(0).unwrap();
            match token {
                Tok::Null => {
                    return vec![JSItem::Ex {
                        expression: Box::new(Expression::Null)
                    }]
                }
                Tok::Float { mut value } => {
                    return vec![JSItem::Ex {
                        expression: Box::new(Expression::Number { value })
                    }];
                }
                Tok::Name { name } => {
                    return vec![JSItem::Ex {
                        expression: Box::new(Expression::Identifier { name: name.clone() })
                    }];
                }
                Tok::String {value} => {
                    return vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: value.clone()})
                    }];
                }
                _ => {}
            }
        }

        let mut js_items = vec![];
        let mut i = 0;
        while i < tokens.len() - 1 {
            let token = tokens.get(i).unwrap();
            match token {
                Tok::For => {
                    let result = find_end_of_for(i, &tokens);
                    match result {
                        Ok(j) => {
                            let t = tokens[i..=j].to_vec();
                            let f = create_for_statement(t);
                            js_items.push(f);
                            i = j;
                        }
                        Err(_) => {}
                    }
                }
                Tok::Float { value: _ } => {
                    let j = find_end_of_expression(i, &tokens, "float");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Let | Tok::Const => {
                    //assignment
                    let assignment_type = find_end_of_assignment(i, &tokens).unwrap();
                    match assignment_type {
                        AssignmentType::Unknown { end: _ } => {}
                        AssignmentType::ArrowFunction { end } => {
                            let t = tokens[i..=end].to_vec();
                            let assign = create_arrow_function(t);
                            js_items.push(assign);
                            i = end;
                        }
                        AssignmentType::Expression { end } => {
                            let t = tokens[i..=end].to_vec();
                            let ex = create_assignment_expression(t).unwrap();
                            js_items.push(ex);
                            i = end;
                        }
                        AssignmentType::FunctionAssignment { end } => {
                            let t = tokens[i..=end].to_vec();
                            let assign = create_function_assignment(t);
                            js_items.push(assign);
                            i = end;
                        }
                        AssignmentType::ObjectExpression {end} => {
                            let t = tokens[i..=end].to_vec();
                            let exr = create_assignment_expression(t).unwrap();
                            js_items.push(exr);
                            i = end;
                        }
                    }
                }
                Tok::Name { name: _ } => {
                    //expression
                    let j = find_end_of_expression(i, &tokens, "name");
                    let t = tokens[i..=j].to_vec();
                    let exr = create_assignment_expression(t.clone());
                    match exr {
                        Ok(ex) => {
                            js_items.push(ex);
                        }
                        Err(..) => {
                            let ex = create_expression(t);
                            js_items.push(ex);
                        }
                    }
                    i = j;
                }
                Tok::Lpar => {
                    let j = find_end_of_expression(i, &tokens, "lpar");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Lbrace => {
                    let j = find_matching_brace(i, &tokens);
                    let t = tokens[i..=j].to_vec();
                    let st = create_object_expression(t).unwrap();
                    js_items.push(st);
                    i = j;
                }
                Tok::Function => {
                    //function
                    let function_type = find_end_of_function(i, &tokens).unwrap();
                    match function_type {
                        FunctionType::FunctionDeclaration { end } => {
                            let t = tokens[i..=end].to_vec();
                            let func = create_function(t);
                            js_items.push(func);
                            i = end;
                        }
                        FunctionType::FunctionExpression {end} => {
                            let t = tokens[i..=end].to_vec();
                            let func = create_function_expression(t);
                            js_items.push(func);
                            i = end;
                        }
                    }
                }
                Tok::Return => {
                    let t = tokens[i+1..=tokens.len() - 1].to_vec();
                    let mut p = Parser::new();
                    let mut items = p.parse(t);
                    let return_item = items.remove(0);
                    js_items.push(JSItem::St {
                        statement: Box::from(Statement::Return{
                            value: Box::new(return_item)
                        })
                    });
                    for item in items {
                        js_items.push(item);
                    }
                    return js_items;
                }
                _ => {
                    i += 1;
                }
            }
        }

        return js_items;
    }

    pub fn new() -> Parser {
        Parser {
            ast_tree: vec![],
        }
    }
}