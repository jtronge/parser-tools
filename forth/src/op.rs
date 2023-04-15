use crate::StackItem;

pub trait Operation {
    fn run(&self, stack: &mut Vec<StackItem>);
}
