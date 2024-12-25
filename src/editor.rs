use std::io::Error;
use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;

use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_exit: bool
}

impl Editor{
    pub const fn default() -> Editor {
        Self{should_exit: false}
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {

        loop {
            self.refresh_screen()?;

            if self.should_exit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 * 2 {
                Self::draw_welcome()?;
            }            
            Self::draw_empty_row()?;

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome() -> Result<(), Error>  {
        let Size {width, ..} = Terminal::size()?;
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let padding = (width as usize - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width as usize);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

 
}
