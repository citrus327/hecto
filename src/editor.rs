use std::io::{self};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

fn die(e: &std::io::Error) {
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
    pub fn run(&self) {
        // 修改terminal的cooked mode为raw mode，即输入后不需要按回车响应用户输入
        // termion会修改老的stdout, 使用_stdout来hold之前老的，否则新的termion会属于unowned, 则会被移除，直接进入cooked mode
        let _stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
            // match self.process_keypress() {
            //     Ok(key) => println!("{:?}\r", key),
            //     Err(error) => die(&error),
            // }
        }
    }
    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program Exited!"),
            _ => (),
        };

        Ok(())
    }
}
