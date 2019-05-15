use crate::{Result, RenderContext};

pub struct GraphicsEngine;

pub trait GraphicsRenderable {
    fn render_graphics(&mut self, engine: &mut GraphicsEngine, context: &RenderContext) -> Result<()>;
}