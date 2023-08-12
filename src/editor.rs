use std::io::{self};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

fn die(e: &std::io::Error) {
    panic!("{}", &e);
}

impl Editor {
    pub fn run(&self) {
        // 修改terminal的cooked mode为raw mode，即输入后不需要按回车响应用户输入
        // termion会修改老的stdout, 使用_stdout来hold之前老的，否则新的termion会属于unowned, 则会被移除，直接进入cooked mode
        let _stdout = io::stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        /*
                         * 判断是否为控制按键输入
                         * Control characters are non-printable characters that we don’t want to print to the screen.
                         * ASCII codes 0–31 are all control characters, and 127 is also a control character.
                         * ASCII codes 32–126 are all printable
                         */
                        if c.is_control() {
                            println!("{c:?}\r");
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?}\r"),
                },
                Err(e) => die(&e),
            }
        }
    }
    pub fn default() -> Self {
        Self {}
    }
}
