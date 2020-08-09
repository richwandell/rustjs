pub trait JsToken {
    fn set_name(&mut self, name: String);
    fn print_name(&self);
    fn get_type(&self) -> String;
}