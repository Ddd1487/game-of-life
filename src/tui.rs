use std::io::{Stdout, Write, stdout};

use crossterm::{
    self,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use crate::{
    Map,
    State::{Alive, Dead},
    rules::{convert_coord_to_index, toggle_state},
};
pub struct TerminalGuard {
    out: Stdout,
}
pub struct TerminalInfo {
    pub width: isize,
    pub height: isize,
    pub cursor_x: isize,
    pub cursor_y: isize,
}
impl TerminalInfo {
    pub fn leave_input(&mut self, event: Event, map: &mut Map) -> bool {
        match event {
            Event::Key(key) => matches!(key.code, KeyCode::Char('q')),
            Event::Mouse(mouse) => {
                if matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
                    self.cursor_x = mouse.column as isize;
                    self.cursor_y = mouse.row as isize;

                    if self.cursor_x >= 0
                        && self.cursor_x < self.width
                        && self.cursor_y >= 0
                        && self.cursor_y < self.height
                    {
                        toggle_state(
                            map,
                            convert_coord_to_index(self.cursor_x, self.cursor_y, map.width),
                        );
                    }
                }
                false
            }
            _ => false,
        }
    }
}
impl TerminalGuard {
    pub fn new() -> anyhow::Result<Self> {
        let mut out = stdout();
        enable_raw_mode()?;
        execute!(out, EnableMouseCapture, EnterAlternateScreen)?;
        Ok(Self { out })
    }
    pub fn writer(&mut self) -> &mut Stdout {
        &mut self.out
    }
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(self.out, LeaveAlternateScreen, DisableMouseCapture);
        let _ = disable_raw_mode();
    }
}
pub fn print_map(map: &Map, stdout: &mut Stdout, _terminal: &TerminalInfo) -> anyhow::Result<()> {
    execute!(stdout, Clear(ClearType::All),)?;
    /*
        0 1 2
        3 4 5
        6 7 8

        9 numbers and 3 spaces => size + height + 1 (height is 0, 1, 2)
    */
    for y in 0_isize..(map.height as isize) {
        let mut row_string: String = String::with_capacity(3 * map.width);
        for x in 0_isize..(map.width as isize) {
            match map.cells[convert_coord_to_index(x, y, map.width)] {
                Alive => row_string.push('█'),
                Dead => row_string.push(' '),
            }
        }
        write!(stdout, "{}\r\n", row_string)?;
    }
    //write!(stdout, "{}", map_string)?;
    /*write!(
        stdout,
        "col {}, row {}",
        terminal.cursor_x, terminal.cursor_y
    )?;*/
    stdout.flush()?;

    Ok(())
}
