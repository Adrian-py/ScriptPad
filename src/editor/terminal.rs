use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(CursorPosition { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn get_size() -> Result<TerminalSize, Error> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(TerminalSize { width, height })
    }

    pub fn print(str: &str) -> Result<(), Error> {
        queue!(stdout(), Print(str))?;
        Ok(())
    }

    pub fn move_cursor_to(pos: CursorPosition) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
}
