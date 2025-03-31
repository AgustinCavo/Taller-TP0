pub trait Operation {
    fn add_operand(&mut self, element: i16) -> bool;
    fn make_operation(&mut self) -> i16;
    fn operands(&self) -> usize;
    fn quantity(&self) -> usize;
    fn name(&self) -> &str;
    fn get_operands(&self) -> &Vec<String>;
}
