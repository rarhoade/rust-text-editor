use crossterm::event::{ read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode, Clear, ClearType, size };
use crossterm::cursor::{ MoveTo };
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            self.draw_rows()?;
            MoveTo(0, 0);
        }
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error>{
        let size = size()?;
        for current_row in 0..size.1 {
            print!("~");
            if current_row + 1 < size.1 {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
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
}