use termion::event::Key;

use crate::{Terminal, VERSION};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", &e);
}

impl Editor {
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
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        };

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye./r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_welcome_messages(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.get_size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.get_size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == 0 {
                self.draw_welcome_messages()
            } else {
                println!("~\r")
            }
        }
    }
}
