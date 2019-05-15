use crate::{GraphicsEngine, GraphicsRenderable, Renderable, RenderContext, Result, TerminalEngine, TerminalRenderable};
use crate::renderer::Renderer;

// TODO: Not sure whether this thing is useful, probably not

pub struct Component<S, T, G> where T: TerminalRenderable, G: GraphicsRenderable {
    state: S,
    renderer: Renderer<T, G>,
}

impl<S, T, G> Renderable for Component<S, T, G> where T: TerminalRenderable, G: GraphicsRenderable {}

impl<S, T, G> TerminalRenderable for Component<S, T, G> where T: TerminalRenderable, G: GraphicsRenderable {
    fn render_terminal(&mut self, engine: &mut TerminalEngine, context: &RenderContext) -> Result<()> {
        if let Renderer::Terminal(mut renderer) = &self.renderer {
            renderer.render_terminal(engine, context)
        } else {
            panic!("duh")
        }
        Ok(())
    }
}

impl<S, T, G> GraphicsRenderable for Component<S, T, G> where T: TerminalRenderable, G: GraphicsRenderable {
    fn render_graphics(&mut self, engine: &mut GraphicsEngine, context: &RenderContext) -> Result<()> {
        if let Renderer::Graphics(mut renderer) = &self.renderer {
            renderer.render_graphics(engine, context)
        } else {
            panic!("duh")
        }
        Ok(())
    }
}