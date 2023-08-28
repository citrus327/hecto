use std::env;

use crate::{row::Row, Document, Terminal};
use termion::color;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);

// (0,0) is at the top left of the screen
#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", &e);
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error)
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }

            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown => self.move_cursor(pressed_key),
            _ => (),
        };
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let size = self.terminal.get_size();
        let width = size.width as usize;
        let height = size.height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye./r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            // Terminal::cursor_position(&self.cursor_position);
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_welcome_messages(&self) {
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");
        let width = self.terminal.get_size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.get_size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{row}\r");
    }
    fn draw_rows(&self) {
        let height = self.terminal.get_size().height;

        for row_index in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.get_row(row_index as usize + self.offset.y) {
                self.draw_row(row)
            } else if self.document.is_empty() && row_index == height / 3 {
                self.draw_welcome_messages();
            } else {
                println!("~\r");
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.get_size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len() as usize;
        let mut width = if let Some(row) = self.document.get_row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height as usize {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x > 0 {
                    x = x - 1;
                } else if y > 0 {
                    y = y - 1;
                    if let Some(row) = self.document.get_row(y) {
                        x = row.len()
                    } else {
                        x = 0
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    height
                }
            }
            _ => (),
        };

        width = if let Some(row) = self.document.get_row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.get_size().width as usize;

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let mut status: String = String::new();

        status = format!("{} - {} lines", file_name, self.document.len());

        let total_len = status.len() + line_indicator.len();

        if width > total_len {
            status.push_str(&" ".repeat(width - total_len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
    }
}
