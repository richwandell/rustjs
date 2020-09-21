use crate::parser::symbols::JSItem;


pub(crate) fn bin_add(a: JSItem, b: JSItem) -> Result<JSItem, ()> {
    match a {
        JSItem::Number { value: v1 } => {
            match b {
                JSItem::Number { value: v2 } => {
                    Ok(JSItem::Number { value: v1 + v2 })
                }
                JSItem::String {value: v2} => {
                    let mut value = v1.to_string();
                    value.push_str(&v2);
                    Ok(JSItem::String {value})
                }
                _ => Err(())
            }
        }
        JSItem::String {value: mut v1 } =>{
            match b {
                JSItem::Number {value: v2} => {
                    let value = v2.to_string();
                    v1.push_str(&value);
                    Ok(JSItem::String {value: v1})
                }
                JSItem::String {value: v2} => {
                    v1.push_str(&v2);
                    Ok(JSItem::String {value: v1})
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_sub(a: JSItem, b: JSItem) -> Result<JSItem, ()> {
    match a {
        JSItem::Number { value: v1 } => {
            match b {
                JSItem::Number { value: v2 } => {
                    Ok(JSItem::Number { value: v1 - v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_mul(a: JSItem, b: JSItem) -> Result<JSItem, ()> {
    match a {
        JSItem::Number { value: v1 } => {
            match b {
                JSItem::Number { value: v2 } => {
                    Ok(JSItem::Number { value: v1 * v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_div(a: JSItem, b: JSItem) -> Result<JSItem, ()> {
    match a {
        JSItem::Number { value: v1 } => {
            match b {
                JSItem::Number { value: v2 } => {
                    Ok(JSItem::Number { value: v1 / v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}

pub(crate) fn bin_less(a: JSItem, b: JSItem) -> Result<JSItem, ()> {
    match a {
        JSItem::Number { value: v1 } => {
            match b {
                JSItem::Number { value: v2 } => {
                    Ok(JSItem::Bool { value: v1 < v2 })
                }
                _ => Err(())
            }
        }
        _ => Err(())
    }
}
