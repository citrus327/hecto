mod document;
mod row;
mod terminal;
use editor::Editor;
use termion::color;
// 将terminal下的Terminal暴露至crate scope
pub use document::Document;
pub use terminal::Terminal;

mod editor;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{}Hecto is running!", color::Fg(color::Green));
    Editor::default().run()
}
