use crossterm::cursor::{Hide, MoveTo};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::style::Print;
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        return Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0,y: 0})?;
        Self::execute()?;
        return Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        return Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        return Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        return Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (.., height) = size()?;
        return Ok(Size { height })
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        return Ok(())
    }
    
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), crossterm::cursor::Show)?;
        return Ok(())
    }
    
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        return Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        return Ok(())
    }
}