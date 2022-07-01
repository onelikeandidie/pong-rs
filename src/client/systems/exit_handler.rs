use std::time::Instant;

use bevy::{prelude::{EventReader, ResMut}, app::AppExit};

use crate::shared::network::{udp::{ClientSocket, construct_packet}, packet_events::{EventMasks, ConnectionData}};

pub fn exit_handler(
    app_exit_events: EventReader<AppExit>,
    mut client_socket: ResMut<ClientSocket>
) {
    if !app_exit_events.is_empty() {
        let sender = client_socket.socket.get_packet_sender();
        // Bytes to sent
        let bytes = EventMasks::Connection(ConnectionData::Disconnect).to_bytes();
        let packet = construct_packet(bytes, client_socket.server_addr);
        sender.send(packet).unwrap();
        // Poll the server lol
        client_socket.socket.manual_poll(Instant::now());
    }
}