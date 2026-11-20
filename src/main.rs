use crate::{terminal_job::game_in_terminal, tui::TerminalInfo};
mod rules;
mod terminal_job;
mod test_functions;
mod tui;

#[derive(Clone)]
pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<State>,
    size: usize,
}
#[derive(Clone, PartialEq, Copy)]
pub enum State {
    Dead,
    Alive,
}
//TODO
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Testing
    //test(10,10,20);
    game_in_terminal(300)?;
    Ok(())
}
