use crate::{GraphicsEngine, GraphicsRenderable, Renderable, Result, TerminalEngine, TerminalRenderable, RenderContext};

pub struct Text<'a> {
    text: &'a str
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Text {
        Text { text }
    }
}

impl<'a> Renderable for Text<'a> {}

impl<'a> TerminalRenderable for Text<'a> {
    fn render_terminal(&mut self, engine: &mut TerminalEngine, context: &RenderContext) -> Result<()> {
        engine.text(self.text, context)
    }
}

impl<'a> GraphicsRenderable for Text<'a> {
    fn render_graphics(&mut self, _engine: &mut GraphicsEngine, _context: &RenderContext) -> Result<()> {
        unimplemented!()
    }
}