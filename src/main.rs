use std::{error::Error, io::stdout, process::exit, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, poll, read}, execute, terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size}};

use crate::{test_functions::test, tui::{TerminalInfo, print_map}};
mod rules;
mod test_functions;
mod tui;
mod terminal_job;

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
//TODO
fn main() -> Result<(), Box<dyn std::error::Error>>{
    //Testing
    //test(10,10,20);
    
    Ok(())
}