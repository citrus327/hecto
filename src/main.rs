mod terminal;
use editor::Editor;
// 将terminal下的Terminal暴露至crate scope
pub use terminal::Terminal;
use termion::color;

mod editor;

fn main() {
    println!("{}Hecto is running!", color::Fg(color::Green));
    Editor::default().run()
}
