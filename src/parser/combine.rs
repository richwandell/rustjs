use crate::parser::symbols::{Expression, Operator, JSItem};
use crate::lexer::js_token::Tok;
use crate::parser::symbols::Expression::Identifier;

pub(crate) fn combine_string(last_exp: Expression, value: String) -> Expression {
    match last_exp {
        Expression::None => {
            return Expression::Literal { value };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_dot(last_exp: Expression, tok: Tok) -> Expression {
    match last_exp {
        Expression::CallExpression { callee, arguments } => {
            match *callee {
                Expression::Identifier { name } => {
                    return Expression::CallExpression {
                        callee: Box::new(Expression::MemberExpression {
                            object: Box::new(Expression::None),
                            property: Box::new(Expression::Identifier { name }),
                        }),
                        arguments,
                    };
                }
                Expression::MemberExpression { object, property } => {
                    let new_object = combine_dot(*object, tok);

                    return Expression::CallExpression {
                        callee: Box::new(Expression::MemberExpression {
                            object: Box::from(new_object),
                            property,
                        }),
                        arguments,
                    };
                }
                _ => {}
            }
        }
        Expression::MemberExpression { object, property } => {
            match *object {
                Expression::Identifier { name } => {
                    let new_object = combine_dot(Expression::Identifier { name }, tok);
                    return Expression::MemberExpression {
                        object: Box::from(new_object),
                        property,
                    };
                }
                Expression::None => {}
                Expression::MemberExpression { object, property } => {
                    let new_object = combine_dot(*object, tok);
                    let new_expression = Expression::MemberExpression {
                        object: Box::new(new_object),
                        property,
                    };
                    return new_expression;
                }
                _ => {}
            }
        }
        Expression::Identifier { name } => {
            return Expression::MemberExpression {
                object: Box::new(Expression::None),
                property: Box::new(Identifier { name }),
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_name(last_exp: Expression, name: String) -> Expression {
    match last_exp {
        Expression::UpdateExpression {expression} => {
            match *expression {
                Expression::None => {
                    return Expression::UpdateExpression {
                        expression: Box::from(Expression::Identifier {name})
                    }
                }
                _ => {}
            }
        }
        Expression::Binop {a, op, b} => {
            match op {
                Operator::Add | Operator::Sub | Operator::Mult | Operator::Div | Operator::Less => {
                    let new_a = combine_name(*a, name);
                    return Expression::Binop {
                        a: Box::from(new_a),
                        op,
                        b,
                    };
                }
                _ => {}
            }
        }
        Expression::CallExpression { callee, arguments } => {
            match *callee {
                Expression::None => {
                    return Expression::CallExpression {
                        callee: Box::new(Expression::Identifier { name }),
                        arguments,
                    };
                }
                Expression::MemberExpression { object, property } => {
                    let new_object = combine_name(*object, name);

                    return Expression::CallExpression {
                        callee: Box::new(Expression::MemberExpression {
                            object: Box::from(new_object),
                            property,
                        }),
                        arguments,
                    };
                }
                _ => {}
            }
        }
        Expression::Identifier { name } => {
            return Expression::MemberExpression {
                object: Box::new(Expression::None),
                property: Box::new(Identifier { name }),
            };
        }
        Expression::None => {
            return Expression::Identifier { name };
        }
        Expression::MemberExpression { object, property } => {
            let outer_property = property;
            match *object {
                Expression::None => {
                    return Expression::MemberExpression {
                        object: Box::from(combine_name(Expression::None, name)),
                        property: outer_property,
                    };
                }
                Expression::MemberExpression { object, property } => {
                    return Expression::MemberExpression {
                        object: Box::from(Expression::MemberExpression {
                            object: Box::new(combine_name(*object, name)),
                            property,
                        }),
                        property: outer_property,
                    };
                }
                _ => {}
            }
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_call(last_exp: Expression, params: Vec<JSItem>) -> Expression {
    match last_exp {
        Expression::None => {
            return Expression::CallExpression {
                callee: Box::new(Expression::None),
                arguments: params,
            };
        }
        Expression::CallExpression { callee, arguments } => {
            match *callee {
                Expression::None => {
                    return Expression::CallExpression {
                        callee: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::None),
                            arguments: params,
                        }),
                        arguments,
                    };
                }
                Expression::MemberExpression { object, property } => {
                    let new_object = combine_call(*object, params);

                    return Expression::CallExpression {
                        callee: Box::new(Expression::MemberExpression {
                            object: Box::from(new_object),
                            property,
                        }),
                        arguments,
                    };
                }
                _ => {}
            }
        }
        Expression::Identifier { name } => {
            return Expression::MemberExpression {
                object: Box::new(Expression::None),
                property: Box::new(Identifier { name }),
            };
        }
        Expression::MemberExpression { object, property } => {
            let outer_property = property;
            match *object {
                Expression::None => {
                    return Expression::MemberExpression {
                        object: Box::from(combine_call(Expression::None, params)),
                        property: outer_property,
                    };
                }
                Expression::MemberExpression { object, property } => {
                    return Expression::MemberExpression {
                        object: Box::from(Expression::MemberExpression {
                            object: Box::new(combine_call(*object, params)),
                            property,
                        }),
                        property: outer_property,
                    };
                }
                _ => {}
            }
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_float(last_exp: Expression, f_value: f64) -> Expression {
    match last_exp {
        Expression::Binop { a, op, b } => {
            match op {
                Operator::Add | Operator::Sub | Operator::Div | Operator::Mult | Operator::Less => {
                    let new_a = combine_float(*a, f_value);
                    return Expression::Binop {
                        a: Box::from(new_a),
                        op,
                        b,
                    };
                }
                _ => {}
            }
        }
        Expression::None => {
            return Expression::Number { value: f_value };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_plus(last_exp: Expression) -> Expression {
    match last_exp {
        Expression::Identifier {name} => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Add,
                b: Box::new(Expression::Identifier {name})
            }
        }
        Expression::Number { value } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Add,
                b: Box::new(Expression::Number { value }),
            };
        }
        Expression::Binop { a, op, b } => {
            match op {
                Operator::Add | Operator::Sub => {
                    let new_exp = combine_plus(*a);
                    return Expression::Binop {
                        a: Box::from(new_exp),
                        op,
                        b,
                    };
                }
                Operator::Mult | Operator::Div => {
                    return Expression::Binop {
                        a: Box::from(Expression::None),
                        op: Operator::Add,
                        b: Box::from(Expression::Binop {
                            a,
                            op,
                            b,
                        }),
                    };
                }
                _ => {}
            }
        }
        Expression::SubExpression { expression } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Add,
                b: Box::new(Expression::SubExpression { expression }),
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_minus(last_exp: Expression) -> Expression {
    match last_exp {
        Expression::Number { value } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Sub,
                b: Box::new(Expression::Number { value }),
            };
        }
        Expression::Binop { a, op, b } => {
            match op {
                Operator::Sub => {
                    let new_exp = combine_minus(*a);
                    return Expression::Binop {
                        a: Box::from(new_exp),
                        op,
                        b,
                    };
                }
                Operator::Div => {
                    return Expression::Binop {
                        a: Box::from(Expression::None),
                        op: Operator::Sub,
                        b: Box::from(Expression::Binop {
                            a,
                            op,
                            b,
                        }),
                    };
                }
                _ => {}
            }
        }
        Expression::SubExpression { expression } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Sub,
                b: Box::new(Expression::SubExpression { expression }),
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_less(last_exp: Expression) -> Expression {
    match last_exp {
        Expression::Identifier {name} => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Less,
                b: Box::new(Expression::Identifier {name}),
            };
        }
        Expression::Number { value } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Less,
                b: Box::new(Expression::Number { value }),
            };
        }
        Expression::Binop { a, op, b } => {
            match op {
                Operator::Less => {
                    let new_exp = combine_less(*a);
                    return Expression::Binop {
                        a: Box::from(new_exp),
                        op,
                        b,
                    };
                }
                Operator::Div => {
                    return Expression::Binop {
                        a: Box::from(Expression::None),
                        op: Operator::Less,
                        b: Box::from(Expression::Binop {
                            a,
                            op,
                            b,
                        }),
                    };
                }
                _ => {}
            }
        }
        Expression::SubExpression { expression } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Less,
                b: Box::new(Expression::SubExpression { expression }),
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_bslash(last_exp: Expression) -> Expression {
    match last_exp {
        Expression::Number { value } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Div,
                b: Box::new(Expression::Number { value }),
            };
        }
        Expression::Binop { a, op, b } => {
            let new_exp = combine_bslash(*a);
            return Expression::Binop {
                a: Box::from(new_exp),
                op,
                b,
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_star(last_exp: Expression) -> Expression {
    match last_exp {
        Expression::Number { value } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Mult,
                b: Box::new(Expression::Number { value }),
            };
        }
        Expression::SubExpression { expression } => {
            return Expression::Binop {
                a: Box::new(Expression::None),
                op: Operator::Mult,
                b: Box::new(Expression::SubExpression { expression }),
            };
        }
        Expression::Binop { a, op, b } => {
            let new_exp = combine_star(*a);
            return Expression::Binop {
                a: Box::from(new_exp),
                op,
                b,
            };
        }
        _ => {}
    }
    return Expression::None;
}

pub(crate) fn combine_expression(last_exp: Expression, next_expression: Expression) -> Expression {
    match last_exp {
        Expression::Binop { a: _, op, b } => {
            return Expression::Binop {
                a: Box::new(Expression::SubExpression {
                    expression: Box::from(next_expression)
                }),
                op,
                b,
            };
        }
        Expression::None => {
            return Expression::SubExpression { expression: Box::new(next_expression) };
        }
        _ => {}
    }
    return Expression::None;
}