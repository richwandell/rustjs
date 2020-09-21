use std::collections::HashMap;

use crate::parser::symbols::JSItem;
use crate::vm::std::array::create_array;
use crate::vm::std::console::create_console;
use crate::vm::std::function::create_function;
use crate::vm::std::object::create_object;

mod object;
mod array;
pub(crate) mod function;
mod inherit;
pub(crate) mod console;

pub(crate) fn create_std_objects() -> HashMap<String, JSItem> {
    let mut f = HashMap::new();

    f = create_object(f);
    f = create_console(f);
    f = create_function(f);
    f = create_array(f);

    return f;
}