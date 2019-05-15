extern crate terminal_ui;

use terminal_ui::{Engine, GridLayout, Renderable, Result, Text};
use terminal_ui::terminal::crossterm::CrosstermEngine;

pub fn main() -> Result<()> {
    let mut engine = CrosstermEngine::new();
    let mut hello = Text::new("Hello");
    let mut world = Text::new("World");
    let mut container = GridLayout::new(vec!["hello world"], vec![
        ("hello", &mut hello as &mut dyn Renderable),
        ("world", &mut world as &mut dyn Renderable)
    ]);
    engine.render(&mut container)
}