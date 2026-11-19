use crate::{TerminalInfo, tui::print_map, State, Map};
use std::thread::sleep;
use std::{error::Error, io::stdout, process::exit, time::{Duration, Instant}};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, poll, read}, execute, terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size}};

pub fn game_in_terminal(tick_interval: u64) -> Result<(), Box<dyn std::error::Error>>{
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

    let interval = Duration::from_millis(tick_interval);
    let mut tick_now = Instant::now() + interval;
    loop {
        //q stops the simulation
        if poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        //game engine
        

        sleep(tick_now - Instant::now());
        tick_now += interval;
    }

    execute!(
        stdout,
        LeaveAlternateScreen,
    )?;
    disable_raw_mode()?;

    Ok(())
}