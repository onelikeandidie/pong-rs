use std::net::SocketAddr;

use bevy::prelude::{ResMut, App, Plugin};
use laminar::{Socket};

use crate::{shared::{resources::cli_args::CliArgs}, server::{systems::server_update::server_update, resources::{client_data::ClientList, server_socket::ServerSocket}}};

pub struct UDPServerPlugin;

impl Plugin for UDPServerPlugin {
    fn build(&self, app: &mut App) {
        let cli_args = CliArgs::extract_args();
        let port = if let Some(port) = cli_args.map.get("port") {
            port
        } else {
            "27807"
        };
        let host = if let Some(host) = cli_args.map.get("host") {
            host
        } else {
            "127.0.0.1"
        };
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        let socket = Socket::bind(addr).unwrap();
        println!("Server listening on: {}", addr);
        app
            .insert_resource(ServerSocket { 
                socket,
                clients: vec![],
                client_data: ClientList::default()
            })
            .add_startup_system(server_startup)
            .add_system(server_update);
    }
}

pub fn server_startup(mut _socket: ResMut<ServerSocket>) {
    // let packet_sender = socket.get_packet_sender();
    // // Bytes to sent
    // let bytes = vec![0];
    // let unreliable = Packet::unreliable_sequenced(client_address(), bytes, Some(1));
    // packet_sender.send(unreliable).unwrap();
}