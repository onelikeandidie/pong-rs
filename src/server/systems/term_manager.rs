use std::{io::{stdin, stdout, Write}, process, thread, time::Duration};

use bevy::{prelude::*, app::AppExit};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen}, execute, event::{DisableMouseCapture, self, Event, KeyCode, KeyModifiers, poll}};
use tui::widgets::{Borders, Block};

use crate::{server::resources::{server_socket::ServerSocket, term::Term}, shared::components::{thonk::Thonk, ball::Ball, transform::Transform}};

pub fn print_client_data_list(_server_socket: Res<ServerSocket>) {
    //println!("{:?}", server_socket.client_data);
}

pub fn startup_terminal(mut term: ResMut<Term>) {
    term.0.hide_cursor().unwrap();
    term.0.clear().unwrap();
}

pub fn draw_terminal(
    mut term: ResMut<Term>,
    app_exit_events: EventReader<AppExit>,
    thonks: Query<(&Thonk, &Transform)>,
    balls: Query<(&Ball, &Transform)>
) {
    term.0.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(format!("App events stack empty: {}", app_exit_events.is_empty()))
            .borders(Borders::ALL);
        f.render_widget(block, size);
        for (thonk, transform) in thonks.iter() {
            let thonk_box = tui::layout::Rect {
                x: transform.x.round() as u16,
                y: transform.y.round() as u16,
                width: thonk.width.round() as u16,
                height: thonk.height.round() as u16
            };
            f.render_widget(Block::default().borders(Borders::ALL), thonk_box)
        }
    }).unwrap();
}

pub fn update_terminal(
    mut term: ResMut<Term>,
    mut app_exit_events: EventWriter<AppExit>
) {
    if let Ok(true) = poll(Duration::from_secs(0)) {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('c') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app_exit_events.send(AppExit);
                    }
                },
                _ => ()
            }
        }
    }
}

pub fn handle_exit(
    mut term: ResMut<Term>,
    app_exit_events: EventReader<AppExit>
) {
    if !app_exit_events.is_empty() {
        term.0.flush().unwrap();
        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            term.0.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();
        term.0.show_cursor().unwrap();
        process::exit(1);
    }
}