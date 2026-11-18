use std::{error::Error, io::stdout, process::exit, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, poll, read}, execute, terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size}};

use crate::{test_functions::test, tui::{TerminalInfo, print_map}};
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
//TODO
fn main() -> Result<(), Box<dyn std::error::Error>>{
    //Testing
    //test(10,10,20);
    let (width_of_terminal, height_of_terminal) = match size(){
        Ok((w, h)) => (w as u16, h as u16),
        Err(_) => (25, 25),
    };

    let mut terminal = TerminalInfo {
        cursor_x: 0,
        cursor_y: 0,
        height: height_of_terminal as isize,
        width: width_of_terminal as isize,
    };

    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnableMouseCapture,
        EnterAlternateScreen,
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
            print_map(&old_map)?;
        }
    }

    execute!(
        stdout,
        DisableMouseCapture,
    )?;
    execute!(
        stdout,
        LeaveAlternateScreen,
    )?;
    disable_raw_mode()?;

    Ok(())
}