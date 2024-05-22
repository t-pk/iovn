use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

mod app;

fn main() -> io::Result<()> {
  enable_raw_mode()?;
  app::run()?;
  disable_raw_mode()?;
  Ok(())
}
