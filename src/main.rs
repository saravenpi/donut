mod config;
mod models;
mod storage;
mod ui;

use config::Config;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use storage::Storage;
use ui::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }

    let config = Config::load();
    let donut_dir = config.get_donut_dir();

    let storage = Storage::new(donut_dir);
    storage.ensure_dir_exists()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(storage);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"Donut - A simple terminal-based todo list manager

USAGE:
    donut [OPTIONS]

OPTIONS:
    -h, --help       Print help information

CONFIGURATION:
    Config file: ~/.donut.yml

    Example config:
        donut_dir: "~/Documents/todos"

    If no config file exists, todos will be stored in ~/.donut/

KEYBOARD SHORTCUTS:
    Project View:
        ↑/↓ or j/k    Navigate
        Tab           Expand/collapse project
        Space         Toggle task (when expanded)
        Enter         Open project
        n             New project
        d             Delete project
        ?             Toggle help
        q/Esc         Quit

    Todo View:
        ↑/↓ or j/k    Navigate
        Space         Toggle completion
        n             New todo
        e             Edit todo
        d             Delete todo
        Backspace     Back to projects
        ?             Toggle help
        q             Quit
"#
    );
}
