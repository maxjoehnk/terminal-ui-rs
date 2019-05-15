use std::ops::Sub;

use crate::{GraphicsRenderable, Result, TerminalRenderable};

pub(crate) enum Renderer<T, G> where T: TerminalRenderable, G: GraphicsRenderable {
    Terminal(T),
    Graphics(G),
}

pub trait Engine {
    fn render(&mut self, component: &mut Renderable) -> Result<()>;

    fn context(&self) -> Result<RenderContext>;
}

pub trait Renderable: TerminalRenderable + GraphicsRenderable {}

#[derive(Debug, Clone, Copy)]
pub struct RenderContext {
    pub bounding_box: (u16, u16)
}
