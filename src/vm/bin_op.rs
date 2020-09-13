use crate::vm::js_output::JSOutput;


pub(crate) fn bin_add(a: JSOutput, b: JSOutput) -> Result<JSOutput, ()> {
    match a {
        JSOutput::Number { value: v1 } => {
            match b {
                JSOutput::Number { value: v2 } => {
                    Ok(JSOutput::Number { value: v1 + v2 })
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
