use crossterm::terminal::{ disable_raw_mode, enable_raw_mode, Clear, ClearType, size };
use crossterm::execute;
use crossterm::cursor::{MoveTo};
use std::io::stdout;

pub struct Terminal {

}

impl Terminal {
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y));
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
}