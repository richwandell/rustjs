use crate::lexer::js_token::Tok;
use crate::parser::symbols::{Expression, Statement, JSItem};
use crate::parser::combine::{combine_star, combine_bslash, combine_plus, combine_minus,
                             combine_float, combine_dot, combine_name, combine_string,
                             combine_call, combine_expression};
use crate::parser::find::function::{find_function_assignment, find_arrow_function};
use crate::parser::find::assignment::find_end_of_assignment;
use crate::parser::find::matching::{find_matching_paren, find_matching_brace};
use crate::parser::find::expression::find_end_of_expression;
use crate::parser::create::function::{create_function, create_arrow_function, create_function_assignment};
use crate::parser::create::expression::{create_expression, create_assignment_expression};

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
}

// fn find_end_of_for() {
//
// }

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Tok>) -> Vec<JSItem> {
        if tokens.len() == 1 {
            let token = tokens.get(0).unwrap();
            match token {
                Tok::Float { mut value } => {
                    return vec![JSItem::Ex { expression: Box::new(Expression::Number { value }) }];
                }
                Tok::Name { name } => {
                    return vec![JSItem::Ex { expression: Box::new(Expression::Identifier { name: name.clone() }) }];
                }
                _ => {}
            }
        }

        let mut js_items = vec![];
        let mut i = 0;
        while i < tokens.len() - 1 {
            let token = tokens.get(i).unwrap();
            match token {
                // Tok::For {
                //     let j = find_end_of_for(i, &tokens);
                // }
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
                            let ex = create_assignment_expression(t);
                            js_items.push(ex);
                            i = end;
                        }
                        AssignmentType::FunctionAssignment { end } => {
                            let t = tokens[i..=end].to_vec();
                            let assign = create_function_assignment(t);
                            js_items.push(assign);
                            i = end;
                        }
                    }
                }
                Tok::Name { name: _ } => {
                    //expression
                    let j = find_end_of_expression(i, &tokens, "name");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Lpar => {
                    let j = find_end_of_expression(i, &tokens, "lpar");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Function => {
                    //function
                    let j = find_matching_brace(i + 1, &tokens);
                    let t = tokens[i..=j].to_vec();
                    let func = create_function(t);
                    js_items.push(func);
                    i = j;
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