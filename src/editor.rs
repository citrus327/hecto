use std::io::{self, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {
    should_quit: bool,
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", &e);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        // 修改terminal的cooked mode为raw mode，即输入后不需要按回车响应用户输入
        // termion会修改老的stdout, 使用_stdout来hold之前老的，否则新的termion会属于unowned, 则会被移除，直接进入cooked mode
        let _stdout = io::stdout().into_raw_mode().unwrap();

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
        Self { should_quit: false }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        };

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye./r");
        }
        io::stdout().flush()
    }
}
