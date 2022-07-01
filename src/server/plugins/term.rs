use std::io;

use bevy::prelude::*;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen}, execute, event::EnableMouseCapture};
use tui::{backend::CrosstermBackend, Terminal};

use crate::server::{systems::term_manager::{update_terminal, startup_terminal, handle_exit, draw_terminal}, resources::term::Term};

pub struct TermPlugin;

impl Plugin for TermPlugin {
    fn build(&self, app: &mut App) {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        app
            .insert_resource(Term(terminal))
            .add_startup_system(startup_terminal)
            .add_system(update_terminal)
            .add_system(draw_terminal)
            .add_system(handle_exit);
    }
}