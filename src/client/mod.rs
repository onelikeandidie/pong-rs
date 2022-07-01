use bevy::{DefaultPlugins, prelude::App};

use self::{plugins::client_startup::UDPClientPlugin, systems::{debug_exit::debug_exit, client_ping::client_ping, exit_handler::exit_handler, setup::setup, network_display_updater::{ping_display_updater, connection_display_updater, NetworkDisplayPlugin}, client_input::{client_input, client_input_sender}, client_update::client_update}, resources::input_data::InputData, events::input_event::InputEvent};

pub mod components {
    pub mod network_displays;
}

pub mod events {
    pub mod input_event;
}

pub mod plugins {
    pub mod client_startup;
    pub mod client;
}

pub mod resources {
    pub mod input_data;
}

pub mod systems {
    pub mod client_input;
    pub mod client_ping;
    pub mod client_update;
    pub mod debug_exit;
    pub mod exit_handler;
    pub mod network_display_updater;
    pub mod setup;
}

pub fn start_client() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UDPClientPlugin)
        .add_plugin(NetworkDisplayPlugin)
        .add_event::<InputEvent>()
        .insert_resource(InputData::default())
        .add_startup_system(setup)
        .add_system(client_input)
        .add_system(client_input_sender)
        .add_system(client_ping)
        .add_system(client_update)
        .add_system(debug_exit)
        .add_system(exit_handler)
        .run()
}