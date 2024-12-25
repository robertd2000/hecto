use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;

use terminal::{Position, Size, Terminal};

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

    fn repl(&mut self) -> Result<(), std::io::Error> {

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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Ok(())
    }



    fn draw_rows() -> Result<(), std::io::Error> {
        let Size {height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
