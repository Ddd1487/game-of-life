use std::{io::stdout, time::Duration};

use crossterm::{self, event::{self, Event, KeyCode, poll}, execute};

use crate::{Map, rules::{convert_coord_to_index, toggle_state}};
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