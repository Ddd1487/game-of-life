use std::io::{Stdout, Write, stdout};

use crossterm::{
    self,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => true,
                KeyCode::Up => {
                    if self.cursor_y < self.height - 1 {
                        self.cursor_y -= 1;
                    }
                    false
                }
                KeyCode::Down => {
                    if self.cursor_y > 1 {
                        self.cursor_y += 1;
                    }
                    false
                }
                KeyCode::Left => {
                    if self.cursor_x > 1 {
                        self.cursor_x -= 1;
                    }
                    false
                }
                KeyCode::Right => {
                    if self.cursor_x < self.width - 1 {
                        self.cursor_x += 1;
                    }
                    false
                }
                KeyCode::Enter => {
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
                    false
                }
                _ => false,
            },
            Event::Mouse(mouse) => {
                self.cursor_x = mouse.row as isize;
                self.cursor_y = mouse.column as isize;

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
pub fn print_map(map: &Map, stdout: &mut Stdout) -> anyhow::Result<()> {
    execute!(stdout, Clear(ClearType::All),)?;
    /*
        0 1 2
        3 4 5
        6 7 8

        9 numbers and 3 spaces => size + height + 1 (height is 0, 1, 2)
    */
    let mut map_string: String = String::with_capacity(map.size + map.height + 1);
    let mut new_line_iter: usize = 0;
    for iter in 0..map.size {
        match map.cells[iter] {
            Dead => map_string.push(' '),
            Alive => map_string.push('█'),
        };
        // 0 1 2 3 4 5 6 -> when iter = 6, go to 0
        if new_line_iter == map.width {
            new_line_iter = 0;
            map_string.push('\r');
        } else {
            new_line_iter += 1;
        }
    }
    write!(stdout, "{}", map_string)?;
    stdout.flush()?;

    Ok(())
}
