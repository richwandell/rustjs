use crate::ast_interpreter::std::array::create_array;
use crate::ast_interpreter::std::function::create_function;
use crate::ast_interpreter::std::object::create_object;
use crate::ast_interpreter::interpreter::Interpreter;
use crate::ast_interpreter::std::console::create_console;

mod object;
pub(crate) mod array;
pub(crate) mod function;
pub(crate) mod inherit;
pub(crate) mod console;

pub(crate) fn create_std_objects(mut int: Interpreter) -> Interpreter {
    int = create_object(int);
    int = create_console(int);
    int = create_function(int);
    int = create_array(int);
    int
}