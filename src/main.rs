use std::io::{self, Read};
use termion::{color, raw::IntoRawMode};

fn with_ctrl(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    println!("{}Hecto is running!", color::Fg(color::Green));

    // 修改terminal的cooked mode为raw mode，即输入后不需要按回车响应用户输入
    // termion会修改老的stdout, 使用_stdout来hold之前老的，否则新的termion会属于unowned, 则会被移除，直接进入cooked mode
    let _stdout = io::stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c: char = b as char;
                /*
                 * 判断是否为控制按键输入
                 * Control characters are non-printable characters that we don’t want to print to the screen.
                 * ASCII codes 0–31 are all control characters, and 127 is also a control character.
                 * ASCII codes 32–126 are all printable
                 */
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }

                if b == with_ctrl('q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
}
