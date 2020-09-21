use crate::lexer::js_token::Tok;
use crate::parser::find::matching::{find_matching_paren, find_matching_brace};
use crate::parser::parser::{FunctionType, SyntaxError};

pub(crate) fn find_arrow_function(start: usize, tokens: &Vec<Tok>) -> usize {
    return match tokens.get(start).unwrap() {
        Tok::Let | Tok::Const => {
            match tokens.get(start + 1).unwrap() {
                Tok::Name { name: _ } => {
                    match tokens.get(start + 2).unwrap() {
                        Tok::Equal => {
                            match tokens.get(start + 3).unwrap() {
                                Tok::Lpar => {
                                    let j = find_matching_paren(start + 3, tokens);
                                    match tokens.get(j + 1).unwrap() {
                                        Tok::RdoubleArrow => {
                                            match tokens.get(j + 2).unwrap() {
                                                Tok::Lbrace => {
                                                    let j = find_matching_brace(j + 2, tokens);
                                                    j
                                                }
                                                _ => start
                                            }
                                        }
                                        _ => start
                                    }
                                }
                                _ => start
                            }
                        }
                        _ => start
                    }
                }
                _ => start
            }
        }
        _ => start
    };
}

pub(crate) fn find_function_assignment(start: usize, tokens: &Vec<Tok>) -> usize {
    return match tokens.get(start).unwrap() {
        Tok::Let | Tok::Const => {
            match tokens.get(start + 1).unwrap() {
                Tok::Name { name: _ } => {
                    match tokens.get(start + 2).unwrap() {
                        Tok::Equal => {
                            match tokens.get(start + 3).unwrap() {
                                Tok::Function => {
                                    match tokens.get(start + 4).unwrap() {
                                        Tok::Lpar => {
                                            let j = find_matching_paren(start + 4, tokens);
                                            match tokens.get(j + 1).unwrap() {
                                                Tok::Lbrace => {
                                                    let j = find_matching_brace(j + 1, tokens);
                                                    j
                                                }
                                                _ => start
                                            }
                                        }
                                        _ => start
                                    }
                                }
                                _ => start
                            }
                        }
                        _ => start
                    }
                }
                _ => start
            }
        }
        _ => start
    };
}

pub(crate) fn find_end_of_function(start: usize, tokens: &Vec<Tok>) -> Result<FunctionType, SyntaxError> {
    return match tokens.get(start).unwrap() {
        Tok::Function => {
            match tokens.get(start + 1).unwrap() {
                Tok::Name { name: _ } => {
                    match tokens.get(start + 2).unwrap() {
                        Tok::Lpar => {
                            let j = find_matching_paren(start + 2, tokens);
                            match tokens.get(j + 1).unwrap() {
                                Tok::Lbrace => {
                                    let k = find_matching_brace(j, tokens);
                                    Ok(FunctionType::FunctionDeclaration { end: k })
                                }
                                _ => {
                                    Err(SyntaxError::UnexpectedToken { tok: tokens.get(j + 1).unwrap().clone() })
                                }
                            }
                        }
                        _ => {
                            Err(SyntaxError::UnexpectedToken { tok: tokens.get(start + 2).unwrap().clone() })
                        }
                    }
                }
                Tok::Lpar => {
                    let j = find_matching_paren(start + 1, tokens);
                    match tokens.get(j + 1).unwrap() {
                        Tok::Lbrace => {
                            let k = find_matching_brace(j, tokens);
                            Ok(FunctionType::FunctionExpression { end: k })
                        }
                        _ => {
                            Err(SyntaxError::UnexpectedToken { tok: tokens.get(j + 1).unwrap().clone() })
                        }
                    }
                }
                _ => {
                    Err(SyntaxError::UnexpectedToken { tok: tokens.get(start + 1).unwrap().clone() })
                }
            }
        }
        _ => {
            Err(SyntaxError::UnexpectedToken { tok: tokens.get(start).unwrap().clone() })
        }
    }
}