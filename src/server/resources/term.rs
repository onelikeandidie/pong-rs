use std::io::Stdout;

use tui::{Terminal, backend::CrosstermBackend};

pub struct Term(pub Terminal<CrosstermBackend<Stdout>>);