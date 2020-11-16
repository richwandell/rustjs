mod object;
pub(crate) mod console;
// mod function;
// mod inherit;

use crate::vm::vm::Vm;
use crate::vm::std::object::create_object;
use crate::vm::std::console::create_console;
// use crate::vm::std::function::create_function;

pub(crate) fn create_std_objects(mut vm: Vm) -> Vm {
    vm = create_object(vm);
    vm = create_console(vm);
    // vm = create_function(vm);
    // int = create_array(int);
    vm
}