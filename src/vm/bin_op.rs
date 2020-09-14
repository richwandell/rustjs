use crate::vm::js_output::JSOutput;


pub(crate) fn bin_add(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Number { value: v1 + v2 })
                }
                JSOutput::String {value: v2} => {
                    let mut value = v1.to_string();
                    value.push_str(&v2);
                    Ok(JSOutput::String {value})
                }
                _ => Err(())
            }
        }
        JSOutput::String {value: mut v1 } =>{
            match b {
                JSOutput::Number {value: v2} => {
                    let value = v2.to_string();
                    v1.push_str(&value);
                    Ok(JSOutput::String {value: v1})
                }
                JSOutput::String {value: v2} => {
                    v1.push_str(&v2);
                    Ok(JSOutput::String {value: v1})
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_sub(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Number { value: v1 - v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_mul(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Number { value: v1 * v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_div(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Number { value: v1 / v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_less(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Bool { value: v1 < v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}
