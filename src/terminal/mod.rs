use crate::{Engine, Result, RenderContext};

pub mod crossterm;

pub trait TerminalEngine: Engine {
    fn text(&mut self, text: &str, context: &RenderContext) -> Result<()>;

    fn move_cursor(&mut self, x: i16, y: i16) -> Result<()>;
}

pub trait TerminalRenderable {
    fn render_terminal(&mut self, engine: &mut dyn TerminalEngine, context: &RenderContext) -> Result<()>;
}
