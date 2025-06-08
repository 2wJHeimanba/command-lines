use std::{io::{stdout, Write}, time::Duration};

use crossterm::{cursor::{MoveRight, MoveTo, RestorePosition, SavePosition}, event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEventKind}, execute, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};




fn main() -> Result<(), Box<dyn std::error::Error>> {

  enable_raw_mode()?;

  let mut stdout = stdout();
  // execute!(stdout, EnableMouseCapture)?;

  queue!(stdout, Print("hello world"))?;

  queue!(stdout, MoveRight(2))?;

  queue!(stdout, Print("wenjianjia"))?;

  stdout.flush()?;

  loop {
    if poll(Duration::from_millis(30))? {
      let event = read()?;
      if let Event::Key(key_event) = event {
        if let KeyCode::Char(ch) = key_event.code {
          if ch == 'c' && key_event.modifiers == KeyModifiers::CONTROL {
            break;
          }
          if ch == 'c' {
            execute!(stdout, MoveTo(0, 5), Clear(ClearType::FromCursorDown))?;
            // execute!(stdout, RestorePosition)?;
          } else {
            execute!(stdout, Print(ch))?;
          }
        }
      } else if let Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
          MouseEventKind::ScrollDown => {
            execute!(stdout, Print("向下滚动"))?;
          }
          MouseEventKind::ScrollUp => {
            execute!(stdout, Print("向上滚动"))?;
          }
          _ => {

          }
        }
      } else {

      }
    }
  }

  // execute!(stdout, DisableMouseCapture)?;s
  disable_raw_mode()?;


  Ok(())


}