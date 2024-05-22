use crossterm::{
  event::{self, Event, KeyCode, KeyEvent},
  execute,
  terminal::{Clear, ClearType},
};
use std::io::{self, stdout, Write};

use iovn::input_handler;

pub fn run() -> io::Result<()> {
  let mut stdout = stdout();
  execute!(stdout, Clear(ClearType::All))?;

  let mut buffer = String::new();

  loop {
    if let Event::Key(KeyEvent { code, .. }) = event::read()? {
      match code {
        KeyCode::Char(c) => {
          input_handler::viet_char_composer(&mut buffer, c);
          execute!(
            stdout,
            Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
          )?;
          write!(stdout, "{}", buffer)?;
          stdout.flush()?;
        }
        KeyCode::Backspace => {
          buffer.pop();
          execute!(
            stdout,
            Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
          )?;
          write!(stdout, "{}", buffer)?;
          stdout.flush()?;
        }
        KeyCode::Esc => {
          break;
        }
        _ => {}
      }
    }
  }

  Ok(())
}
