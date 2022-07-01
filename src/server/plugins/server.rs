use bevy::prelude::{Plugin, App};
use super::server_startup::UDPServerPlugin;

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UDPServerPlugin);
    }
}