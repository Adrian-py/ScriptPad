use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Command,
};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct CaretPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        let _ = Self::enter_alternate_screen();
        Self::clear_screen()?;
        Self::move_caret_to(CaretPosition { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        let _ = Self::leave_alternate_screen();
        let _ = Self::show_caret();
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = crossterm::terminal::size()?;
        #[allow(clippy::as_conversions)]
        Ok(Size {
            width: width as usize,
            height: height as usize,
        })
    }

    pub fn print(str: &str) -> Result<(), Error> {
        Self::queue_command(Print(str))?;
        Ok(())
    }

    pub fn print_row(row: usize, str: &str) -> Result<(), Error> {
        Self::move_caret_to(CaretPosition { x: 0, y: row })?;
        Self::clear_line()?;
        Self::queue_command(Print(str))?;
        Ok(())
    }

    pub fn move_caret_to(pos: CaretPosition) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
}
