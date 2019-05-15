use crossterm::{Crossterm, Terminal, TerminalCursor};

use crate::{Engine, Renderable, RenderContext, Result};

use super::TerminalEngine;

pub struct CrosstermEngine {
    terminal: Terminal,
    cursor: TerminalCursor,
}

impl CrosstermEngine {
    pub fn new() -> CrosstermEngine {
        let crossterm = Crossterm::new();
        let terminal = crossterm.terminal();
        let cursor = crossterm.cursor();
        CrosstermEngine {
            terminal,
            cursor,
        }
    }
}

impl TerminalEngine for CrosstermEngine {
    fn text(&mut self, text: &str, context: &RenderContext) -> Result<()> {
        let (width, height) = context.bounding_box;
        let text_len = text.len();
        // text.lines()
        self.terminal.write(text)?;
        Ok(())
    }

    fn move_cursor(&mut self, x: i16, y: i16) -> Result<()> {
        if x > 0 {
            self.cursor.move_right(x as u16);
        } else if x < 0 {
            self.cursor.move_left((x * -1) as u16);
        }
        if y > 0 {
            self.cursor.move_down(y as u16);
        } else if y < 0 {
            self.cursor.move_up((y * -1) as u16);
        }

        Ok(())
    }
}

impl Engine for CrosstermEngine {
    fn render(&mut self, component: &mut Renderable) -> Result<()> {
        self.terminal.clear(crossterm::ClearType::All)?;
        let context = self.context()?;
        component.render_terminal(self, &context)
    }

    fn context(&self) -> Result<RenderContext> {
        Ok(RenderContext {
            bounding_box: self.terminal.terminal_size()
        })
    }
}