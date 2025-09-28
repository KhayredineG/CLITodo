use crate::{
    app::{App, AppMode},
    ui::ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

mod app;
mod task;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::<B>(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        app.save();
                        return Ok(());
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.toggle_completed(),
                    KeyCode::Char('a') => {
                        app.adding_subtask = false;
                        app.mode = AppMode::Insert;
                    },
                    KeyCode::Char('d') => app.delete_task(),
                    KeyCode::Char('p') => app.cycle_priority(),
                    KeyCode::Char('D') => app.mode = AppMode::DateInput,
                    KeyCode::Char('s') => {
                        app.adding_subtask = true;
                        app.mode = AppMode::Insert;
                    },
                    KeyCode::Char('/') => app.mode = AppMode::Search,
                    KeyCode::Char('+') => app.zoom_in(),
                    KeyCode::Char('-') => app.zoom_out(),
                    _ => {}
                },
                AppMode::Insert => match key.code {
                    KeyCode::Enter => app.add_task(),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.adding_subtask = false;
                        app.mode = AppMode::Normal;
                    },
                    _ => {}
                },
                AppMode::DateInput => match key.code {
                    KeyCode::Enter => app.set_due_date(),
                    KeyCode::Char(c) => app.date_input.push(c),
                    KeyCode::Backspace => {
                        app.date_input.pop();
                    }
                    KeyCode::Esc => app.mode = AppMode::Normal,
                    _ => {}
                },
                AppMode::Search => match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.mode = AppMode::Normal,
                    KeyCode::Char(c) => app.search_input.push(c),
                    KeyCode::Backspace => {
                        app.search_input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}