use crossterm::event::{
    read,
    Event::{self},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
use terminal::{CaretPosition, Size, Terminal};
use view::View;

mod terminal;
mod view;

pub struct Editor {
    cursor_position: CaretPosition,
    should_exit: bool,
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        let _ = Terminal::print("Goodbye! :D");
    }
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        // Add customization to hook handling
        let current_panic_hook = take_hook();
        set_hook(Box::new(move |info| {
            let _ = Terminal::terminate();
            current_panic_hook(info);
        }));

        Terminal::initialize().unwrap();

        // Initialize editor attributes
        let initial_caret_position: CaretPosition = CaretPosition { x: 2, y: 0 };
        let mut view: View = View::default();
        let args: Vec<String> = env::args().collect::<Vec<String>>(); // retrieve file path to load from arguments
        if let Some(file_path) = args.get(1) {
            view.load(file_path);
        }

        Ok(Self {
            cursor_position: initial_caret_position,
            should_exit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_exit {
                break;
            }

            match read() {
                Ok(event) => {
                    self.handle_event(event);
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }
    pub fn handle_args(&mut self) {
        let args = env::args().collect::<Vec<String>>();

        if let Some(file_path) = args.get(1) {
            self.view.load(file_path);
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.cursor_position);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_event(&mut self, event: Event) {
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
                    self.move_caret(code);
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
    }

    fn move_caret(&mut self, code: KeyCode) {
        let terminal_size = Terminal::size().unwrap_or_default();
        match code {
            KeyCode::Left => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor_position.x = self.cursor_position.x.saturating_add(1);
            }
            KeyCode::Down => {
                self.cursor_position.y = self.cursor_position.y.saturating_add(1);
            }
            KeyCode::Up => {
                self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
            }
            KeyCode::PageDown => {
                self.cursor_position.y = terminal_size.height;
            }
            KeyCode::PageUp => {
                self.cursor_position.y = 0;
            }
            KeyCode::Home => {
                self.cursor_position.x = 0;
            }
            KeyCode::End => {
                self.cursor_position.x = terminal_size.width;
            }
            _ => {}
        }
    }
}
