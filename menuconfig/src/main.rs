use crossterm::{
  event::{self, KeyCode},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
  Terminal,
  backend::CrosstermBackend,
  widgets::{Block, Borders, List, ListItem},
};
use std::io;

fn main() -> io::Result<()>
{
  // Configurar terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let options = vec!["Option 1", "Option 2", "Exit"];
  let mut selected = 0;

  loop {
    terminal.draw(|frame| {
      let size = frame.size();
      let items: Vec<ListItem> = options.iter().map(|&item| ListItem::new(item)).collect();

      let list = List::new(items).block(Block::default().title(" Menu ").borders(Borders::ALL));

      frame.render_widget(list, size);
    })?;

    if let event::Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Up => {
          if selected > 0 {
            selected -= 1;
          }
        }
        KeyCode::Down => {
          if selected < options.len() - 1 {
            selected += 1;
          }
        }
        KeyCode::Enter => {
          if options[selected] == "Exit" {
            break;
          }
          println!("Selected: {}", options[selected]);
        }
        KeyCode::Esc => break,
        _ => {}
      }
    }
  }

  // Restaurar terminal
  disable_raw_mode()?;
  execute!(io::stdout(), LeaveAlternateScreen)?;
  Ok(())
}
