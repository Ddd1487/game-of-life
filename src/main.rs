use crate::{State::Alive, rules::run_generation};

mod rules;
mod test_functions;

#[derive(Clone)]
pub struct Map{
    width: usize,
    height: usize,
    cells: Vec<State>,
    size: usize,
}
#[derive(Clone,PartialEq,Copy)]
pub enum State{
    Dead,
    Alive,
}
fn main() {
    //Testing
    test(10,10,1);
}