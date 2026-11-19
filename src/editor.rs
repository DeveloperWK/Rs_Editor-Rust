use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::io::{Error, stdout};
pub struct Editor {
    should_quit: bool,
}
impl Editor {
    pub const fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self::init().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
