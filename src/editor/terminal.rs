use crossterm::cursor::{Hide, MoveTo, Show};

use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::stdout;
use std::io::{ Error, Write};

pub struct Size{
    pub width: u16,
    pub height: u16
}

pub struct Position{
    pub x: u16,
    pub y: u16
}

pub struct Terminal  {}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::execute()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::execute()?;
        Ok(())
    }
    
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size{width, height})
    }
    pub fn move_cursor_to(Position{x, y}: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn print(text: &str) -> Result<(), Error> {
        queue!(stdout(), Print(text))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(()) 
    }
}