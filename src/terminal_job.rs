use crate::rules::run_generation;
use crate::tui::TerminalGuard;
use crate::{Map, State, TerminalInfo, tui::print_map};
use crossterm::{
    event::{KeyCode, poll, read},
    terminal::size,
};
use std::thread::sleep;
use std::time::{Duration, Instant};
fn get_window_size() -> (u16, u16) {
    let (width_of_terminal, height_of_terminal) = match size() {
        Ok((w, h)) => (w, h),
        Err(_) => (25, 25),
    };
    (width_of_terminal, height_of_terminal)
}
pub fn game_in_terminal(tick_interval: u64) -> anyhow::Result<()> {
    let mut guard = TerminalGuard::new()?;
    let out = guard.writer();

    let (width_of_terminal, height_of_terminal) = get_window_size();
    let mut terminal = TerminalInfo {
        cursor_x: 0,
        cursor_y: 0,
        height: height_of_terminal as isize,
        width: width_of_terminal as isize,
    };
    let width: usize = terminal.width as usize;
    let height: usize = terminal.height as usize;

    let mut old_map = Map {
        width,
        height,
        cells: vec![State::Dead; width * height],
        size: width * height,
    };
    let mut new_map = old_map.clone();

    loop {
        if poll(Duration::from_millis(100))? {
            let event = read()?;
            if terminal.leave_input(event, &mut new_map) {
                break;
            }
            std::mem::swap(&mut old_map, &mut new_map);
            print_map(&old_map, out)?;
        }
    }

    let interval = Duration::from_millis(tick_interval);
    let mut tick_now = Instant::now() + interval;
    loop {
        //q stops the simulation
        if poll(Duration::from_millis(0))?
            && let crossterm::event::Event::Key(key) = read()?
            && key.code == KeyCode::Char('q')
        {
            break;
        }

        //game engine
        run_generation(&old_map, &mut new_map);
        std::mem::swap(&mut old_map, &mut new_map);
        print_map(&old_map, out)?;

        let remaining = tick_now.saturating_duration_since(Instant::now());
        sleep(remaining);
        tick_now += interval;
    }

    Ok(())
}
