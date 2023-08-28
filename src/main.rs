mod document;
mod row;
mod terminal;
use editor::Editor;
// 将terminal下的Terminal暴露至crate scope
pub use document::Document;
pub use terminal::Terminal;

mod editor;

fn main() {
    Editor::default().run()
}
