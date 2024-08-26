use command::Command;
use crossterm::event::{
    read,
    Event::{self},
    KeyEvent, KeyEventKind,
};
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
use terminal::Terminal;
use view::View;

mod command;
mod terminal;
mod view;

pub struct Editor {
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

        Terminal::initialize()?;

        // Initialize editor attributes
        let mut view: View = View::default();
        let args: Vec<String> = env::args().collect::<Vec<String>>(); // retrieve file path to load from arguments
        if let Some(file_path) = args.get(1) {
            view.load(file_path);
        }

        Ok(Self {
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
        let _ = Terminal::move_caret_to(&self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_event(&mut self, event: Event) {
        let should_execute: bool = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_execute {
            return;
        }

        match Command::try_from(event) {
            Ok(command) => match command {
                Command::Quit => self.should_exit = true,
                _ => self.view.handle_command(command),
            },
            Err(error_message) => {
                #[cfg(debug_assertions)]
                {
                    panic!("Command not supported: {error_message}");
                }
            }
        }
    }
}
