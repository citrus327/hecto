use editor::Editor;
use termion::color;

mod editor;

fn main() {
    println!("{}Hecto is running!", color::Fg(color::Green));
    Editor::default().run()
}
