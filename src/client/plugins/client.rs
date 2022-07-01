use bevy::prelude::{Plugin, App};
use super::client_startup::UDPClientPlugin;

struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UDPClientPlugin);
    }
}