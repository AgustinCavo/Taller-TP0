pub trait Operation {
    fn add_operand(&mut self, element: i16) -> bool;
    fn make_operation(&self) -> i16;
    fn operands(&self) -> usize;
    fn quantity(&self) -> usize;
    fn name(&self) -> &str;
}
