use crossterm::event::{
    read,
    Event::{self},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::{env, io::Error};
use terminal::{CursorPosition, Size, Terminal};
use view::View;

mod terminal;
mod view;

pub struct Editor {
    cursor_position: CursorPosition,
    should_exit: bool,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            cursor_position: CursorPosition { x: 2, y: 0 },
            should_exit: false,
            view: View::default(),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();

        let editor_res = self.repl();
        Terminal::terminate().unwrap();
        editor_res.unwrap();
    }

    pub fn handle_args(&mut self) {
        let args = env::args().collect::<Vec<String>>();

        if let Some(file_path) = args.get(1) {
            self.view.load(file_path);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_exit {
                break;
            }

            let event = read()?;
            self.handle_event(event)?;
        }

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(CursorPosition { x: 0, y: 0 })?;
        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! :D")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(self.cursor_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_exit = true;
                }
                (KeyCode::Char(c), _) => Terminal::print(&c.to_string())?,
                (
                    KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::Home
                    | KeyCode::End,
                    _,
                ) => {
                    self.move_caret(code)?;
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                // Cast u16 to usize, and ignore clippy warnings
                #[allow(clippy::as_conversions)]
                let w_usize = w as usize;
                #[allow(clippy::as_conversions)]
                let h_usize = h as usize;

                self.view.window_resize(Size {
                    width: w_usize,
                    height: h_usize,
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn move_caret(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Left => {
                self.cursor_position.x = self.cursor_position.x.max(3).saturating_sub(1);
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::Right => {
                self.cursor_position.x = self
                    .cursor_position
                    .x
                    .min(Terminal::size()?.width.saturating_sub(2))
                    .saturating_add(1);
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::Down => {
                self.cursor_position.y = self
                    .cursor_position
                    .y
                    .min(Terminal::size()?.height.saturating_sub(2))
                    .saturating_add(1);
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::Up => {
                self.cursor_position.y = self.cursor_position.y.max(1).saturating_sub(1);
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::PageDown => {
                self.cursor_position.y = Terminal::size()?.height;
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::PageUp => {
                self.cursor_position.y = 0;
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::Home => {
                self.cursor_position.y = 2;
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            KeyCode::End => {
                self.cursor_position.y = Terminal::size()?.width;
                Terminal::move_cursor_to(self.cursor_position)?;
            }
            _ => {}
        }

        Ok(())
    }
}
