use std::{fmt::Alignment, io::{Write, stdout}, time::Duration};

use crossterm::{self, cursor::MoveTo, event::{self, Event, KeyCode, poll}, execute, terminal::{Clear, ClearType}};

use crate::{Map, State::{Alive, Dead}, rules::{convert_coord_to_index, toggle_state}};
pub struct TerminalInfo{
    pub width: isize,
    pub height: isize,
    pub cursor_x: isize,
    pub cursor_y: isize,
}
impl TerminalInfo{
    pub fn leave_input(&mut self, event: Event, map: &mut Map) -> bool{
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return true,
                    KeyCode::Up => {
                        if self.cursor_y < self.height - 1 {
                            self.cursor_y += 1;
                        }
                        return false;
                    }
                    KeyCode::Down => {
                        if self.cursor_y > 1 {
                            self.cursor_y -= 1;
                        }
                        return false;
                    }
                    KeyCode::Left => {
                        if self.cursor_x > 1 {
                            self.cursor_x -= 1;
                        }
                        return false;
                    }
                    KeyCode::Right => {
                        if self.cursor_x < self.width -1 {
                            self.cursor_x += 1;
                        }
                        return false;
                    }
                    KeyCode::Enter => {
                        toggle_state(map, convert_coord_to_index(self.cursor_x, self.cursor_y, map.width));
                        return false;
                    }
                    _ => return false,
                }
            }
            Event::Mouse(mouse) => {
                self.cursor_x = mouse.row as isize;
                self.cursor_y = mouse.column as isize;

                if self.cursor_x >= 0 && self.cursor_x < self.width
                    && self.cursor_y >= 0 && self.cursor_y < self.height{
                        toggle_state(map, convert_coord_to_index(self.cursor_x, self.cursor_y, map.width));
                    }
                return false;
            }
            _ => return false,
        }
    } 
}
//TODO
pub fn print_map(map: &Map) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0,0),
    )?;

    let mut map_string: String = String::with_capacity(map.size + map.height);
    let mut new_line_iter: usize = 0;
    for iter in 0..map.size {
        match map.cells[iter] {
            Dead => map_string.push(' '),
            Alive => map_string.push('█'),
        };
        if new_line_iter == map.width {
            new_line_iter = 0;
            map_string.push_str("\n");
        }
        else {
            new_line_iter += 1;
        }
    }
    print!("{}", map_string);
    stdout.flush()?;

    Ok(())
}