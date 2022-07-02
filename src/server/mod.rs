use std::time::Duration;

use bevy::{app::{ScheduleRunnerSettings, ScheduleRunnerPlugin}, core::CorePlugin, prelude::App};

use crate::shared::plugins::cli_plugin::CliPlugin;

use self::{plugins::{server_startup::UDPServerPlugin, physics::PhysicsPlugin}, systems::{game_systems::setup, state_management::update_player_thonks}};

pub mod components {
    pub mod server_bundles;
}

pub mod events {
    pub mod physics;
}

pub mod plugins {
    pub mod physics;
    pub mod server_startup;
    pub mod server;
}

pub mod resources {
    pub mod client_data;
    pub mod server_socket;
}

pub mod systems {
    pub mod game_systems;
    pub mod physics_system;
    pub mod server_update;
    pub mod state_management;
}

pub fn start_server() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(
            Duration::from_secs_f32(1.0 / 140.0) // 140 tick rate
        ))
        .add_plugin(ScheduleRunnerPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(CliPlugin)
        .add_plugin(UDPServerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .add_system(update_player_thonks)
        .run();
}