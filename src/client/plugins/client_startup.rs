use std::net::SocketAddr;

use bevy::{prelude::{App, Plugin, Res}, core::Timer};
use laminar::{Socket, Packet};

use crate::{shared::{network::{udp::{ClientSocket, server_address, NetworkStats}, packet_events::{EventMasks, ConnectionData}}, resources::cli_args::CliArgs}, client::systems::{client_update::client_update, client_ping::PingTimer}};

pub struct UDPClientPlugin;

impl Plugin for UDPClientPlugin {
    fn build(&self, app: &mut App) {
        let cli_args = CliArgs::extract_args();
        let cport = if let Some(port) = cli_args.map.get("cport") {
            port
        } else {
            "27808"
        };
        let chost = if let Some(host) = cli_args.map.get("chost") {
            host
        } else {
            "127.0.0.1"
        };
        let client_addr: SocketAddr = format!("{}:{}", chost, cport).parse().unwrap();
        let sport = if let Some(port) = cli_args.map.get("sport") {
            port
        } else {
            "27807"
        };
        let shost = if let Some(host) = cli_args.map.get("shost") {
            host
        } else {
            "127.0.0.1"
        };
        let server_addr: SocketAddr = format!("{}:{}", shost, sport).parse().unwrap();
        let socket = Socket::bind(client_addr).unwrap();
        println!("Client listening on: {}\nServer on: {}", client_addr, server_addr);
        app
            .insert_resource(ClientSocket { 
                socket, 
                server_addr,
                network: NetworkStats::default()
            })
            .insert_resource(PingTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(client_startup)
            .add_system(client_update);
    }
}

pub fn client_startup(socket: Res<ClientSocket>) {
    //let packet_sender = socket.socket.get_packet_sender();
    //// Bytes to sent
    //let bytes = EventMasks::Connection(ConnectionData::Connect).to_bytes();
    //let unreliable = Packet::reliable_unordered(server_address(), bytes);
    //packet_sender.send(unreliable).unwrap();
}