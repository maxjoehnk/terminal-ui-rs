extern crate terminal_ui;

use terminal_ui::{Engine, Result, Text};
use terminal_ui::terminal::crossterm::CrosstermEngine;

pub fn main() -> Result<()> {
    let mut engine = CrosstermEngine::new();
    let mut text = Text::new("Hello World");
    engine.render(&mut text)
}