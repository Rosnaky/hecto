#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo, clippy::restriction)]
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}