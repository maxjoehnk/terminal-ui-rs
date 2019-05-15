use crate::{GraphicsEngine, GraphicsRenderable, Renderable, RenderContext, Result, TerminalEngine, TerminalRenderable};

pub struct Padding<'c, C> where C: Renderable {
    padding: u16,
    child: &'c mut C,
}

impl<'c, C> Padding<'c, C> where C: Renderable {
    pub fn new(padding: u16, child: &'c mut C) -> Padding<C> {
        Padding {
            padding,
            child,
        }
    }
}

impl<'c, C> Renderable for Padding<'c, C> where C: Renderable {}

impl<'c, C> TerminalRenderable for Padding<'c, C> where C: Renderable {
    fn render_terminal(&mut self, engine: &mut TerminalEngine, context: &RenderContext) -> Result<()> {
        let (width, height) = context.bounding_box;
        let bounding_box = (width - (self.padding * 2), height - (self.padding * 2));
        engine.move_cursor(self.padding as i16, self.padding as i16)?;
        self.child.render_terminal(engine, &RenderContext {
            bounding_box
        })
    }
}

impl<'c, C> GraphicsRenderable for Padding<'c, C> where C: Renderable {
    fn render_graphics(&mut self, _engine: &mut GraphicsEngine, _context: &RenderContext) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Engine, GraphicsEngine, GraphicsRenderable, GridLayout, Padding, Renderable, RenderContext, Result, TerminalEngine, TerminalRenderable};

    #[derive(Default)]
    struct MockTerminalEngine {
        cursor_movement: (i16, i16)
    }

    impl Engine for MockTerminalEngine {
        fn render(&mut self, component: &mut Renderable) -> Result<()> {
            unimplemented!()
        }

        fn context(&self) -> Result<RenderContext> {
            unimplemented!()
        }
    }

    impl TerminalEngine for MockTerminalEngine {
        fn text(&mut self, text: &str, context: &RenderContext) -> Result<()> {
            unimplemented!()
        }

        fn move_cursor(&mut self, x: i16, y: i16) -> Result<()> {
            self.cursor_movement = (x, y);
            Ok(())
        }
    }

    #[derive(Default)]
    struct TestComponent {
        bounding_box: (u16, u16)
    }

    impl Renderable for TestComponent {}

    impl TerminalRenderable for TestComponent {
        fn render_terminal(&mut self, _engine: &mut TerminalEngine, context: &RenderContext) -> Result<()> {
            self.bounding_box = context.bounding_box;
            Ok(())
        }
    }

    impl GraphicsRenderable for TestComponent {
        fn render_graphics(&mut self, _engine: &mut GraphicsEngine, _context: &RenderContext) -> Result<()> {
            unimplemented!()
        }
    }


    #[test]
    fn render_terminal_should_subtract_twice_the_padding_from_the_bounding_box() -> Result<()> {
        let mut engine = MockTerminalEngine::default();
        let context = RenderContext {
            bounding_box: (10, 10)
        };
        let mut component = TestComponent::default();
        let mut layout = Padding::new(2, &mut component);

        layout.render_terminal(&mut engine, &context)?;

        assert_eq!(component.bounding_box, (6, 6));
        Ok(())
    }

    #[test]
    fn render_terminal_should_move_the_cursor_by_the_amount_of_padding() -> Result<()> {
        let mut engine = MockTerminalEngine::default();
        let context = RenderContext {
            bounding_box: (10, 10)
        };
        let mut component = TestComponent::default();
        let mut layout = Padding::new(2, &mut component);

        layout.render_terminal(&mut engine, &context)?;

        assert_eq!(engine.cursor_movement, (2, 2));
        Ok(())
    }
}