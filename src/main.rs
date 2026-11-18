use std::{error::Error, io::stdout, process::exit, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, poll, read}, execute, terminal::{disable_raw_mode, enable_raw_mode, size}};

use crate::{test_functions::test, tui::{TerminalInfo}};
mod rules;
mod test_functions;
mod tui;

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
fn main() -> Result<(), Box<dyn std::error::Error>>{
    //Testing
    //test(10,10,20);
    let mut width: u16;
    let mut height: u16;
    let (width, height) = match size(){
        Ok((w, h)) => (w as u16, h as u16),
        Err(_) => (25, 25),
    };

    let mut terminal = TerminalInfo {
        cursor_x: 0,
        cursor_y: 0,
        height: height as isize,
        width: width as isize,
    };

    match enable_raw_mode() {
        Ok(()) => (),
        Err(_) => exit(1),
    };
    execute!(
        stdout(),
        EnableMouseCapture,
    )?;

    let width: usize = terminal.width as usize;
    let height: usize = terminal.height as usize;
    let mut old_map = Map{
        width,
        height,
        cells: vec![State::Dead; width * height],
        size: width * height,
    };
    let mut new_map = old_map.clone();

    loop {
        if poll(Duration::from_millis(100))? {
            let event = read()?;
            if terminal.leave_input(event, &mut old_map){
                break;
            }
        }
    }

    execute!(
        stdout(),
        DisableMouseCapture,
    )?;
    match disable_raw_mode() {
        Ok(()) => (),
        Err(_) => exit(2),
    };

    Ok(())
}