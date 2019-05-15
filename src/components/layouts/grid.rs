use crate::{GraphicsEngine, GraphicsRenderable, Renderable, RenderContext, Result, TerminalEngine, TerminalRenderable};

pub struct GridLayout<'l> {
    layout: Vec<&'l str>,
    components: Vec<(&'l str, &'l mut dyn Renderable)>,
}

impl<'l> GridLayout<'l> {
    pub fn new(layout: Vec<&'l str>, components: Vec<(&'l str, &'l mut dyn Renderable)>) -> GridLayout<'l> {
        GridLayout {
            layout,
            components,
        }
    }
}

impl<'l> Renderable for GridLayout<'l> {}

impl<'l> TerminalRenderable for GridLayout<'l> {
    fn render_terminal(&mut self, engine: &mut dyn TerminalEngine, context: &RenderContext) -> Result<()> {
        let (width, height) = context.bounding_box;
        let rows = self.layout[0].split(" ").collect::<Vec<_>>().len() as u16;
        let columns = self.layout.len() as u16;
        for (space, component) in self.components.iter_mut() {
            let local_context = RenderContext {
                bounding_box: (width / rows, height / columns)
            };
            component.render_terminal(engine, &local_context)?;
        }

        Ok(())
    }
}

impl<'l> GraphicsRenderable for GridLayout<'l> {
    fn render_graphics(&mut self, _engine: &mut GraphicsEngine, _context: &RenderContext) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Engine, GraphicsEngine, GraphicsRenderable, GridLayout, Renderable, RenderContext, Result, TerminalEngine, TerminalRenderable};

    struct MockTerminalEngine;

    impl Engine for MockTerminalEngine {
        fn render(&mut self, component: &mut Renderable) -> Result<()> {
            unimplemented!()
        }

        fn context(&self) -> Result<RenderContext> {
            unimplemented!()
        }
    }

    impl TerminalEngine for MockTerminalEngine {
        fn text(&mut self, _text: &str, _context: &RenderContext) -> Result<()> {
            unimplemented!()
        }

        fn move_cursor(&mut self, _x: i16, _y: i16) -> Result<()> {
            unimplemented!()
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
    fn render_terminal_should_give_a_single_child_the_complete_bounding_box() -> Result<()> {
        let mut engine = MockTerminalEngine;
        let context = RenderContext {
            bounding_box: (10, 10)
        };
        let mut component = TestComponent::default();
        let mut layout = GridLayout::new(vec!["test"], vec![
            ("test", &mut component)
        ]);

        layout.render_terminal(&mut engine, &context)?;

        assert_eq!(component.bounding_box, (10, 10));
        Ok(())
    }

    #[test]
    fn render_terminal_should_give_two_side_by_side_children_equal_sized_halves() -> Result<()> {
        let mut engine = MockTerminalEngine;
        let context = RenderContext {
            bounding_box: (10, 10)
        };
        let mut component1 = TestComponent::default();
        let mut component2 = TestComponent::default();
        let mut layout = GridLayout::new(vec!["1 2"], vec![
            ("1", &mut component1),
            ("2", &mut component2)
        ]);

        layout.render_terminal(&mut engine, &context)?;

        assert_eq!(component1.bounding_box, (5, 10));
        assert_eq!(component2.bounding_box, (5, 10));
        Ok(())
    }

    #[test]
    fn render_terminal_should_give_two_children_above_one_another_equal_sized_halves() -> Result<()> {
        let mut engine = MockTerminalEngine;
        let context = RenderContext {
            bounding_box: (10, 10)
        };
        let mut component1 = TestComponent::default();
        let mut component2 = TestComponent::default();
        let mut layout = GridLayout::new(vec!["1", "2"], vec![
            ("1", &mut component1),
            ("2", &mut component2)
        ]);

        layout.render_terminal(&mut engine, &context)?;

        assert_eq!(component1.bounding_box, (10, 5));
        assert_eq!(component2.bounding_box, (10, 5));
        Ok(())
    }
}