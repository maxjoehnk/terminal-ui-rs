extern crate terminal_ui;

use terminal_ui::{Engine, Padding, Result, Text};
use terminal_ui::terminal::crossterm::CrosstermEngine;

pub fn main() -> Result<()> {
    let mut engine = CrosstermEngine::new();
    let mut text = Text::new("Hello World\nA long line to fill your terminal with. This should be way longer. Maybe I should've copied lorem ipsum");
    let mut padding = Padding::new(8, &mut text);
    engine.render(&mut padding)
}