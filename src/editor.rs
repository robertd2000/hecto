use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {}

impl Editor{
    pub fn default() -> Editor {
        Editor{}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            println!("Error: {err}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode().unwrap();

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            }
        }
        disable_raw_mode()?; 
        Ok(())
    }
}
