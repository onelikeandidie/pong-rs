use bevy::{prelude::{Res, ResMut}, core::{Timer, Time}};
use laminar::Packet;

use crate::shared::network::{udp::{ClientSocket, server_address}, packet_events::{EventMasks, PingData, ConnectionData}};

pub struct PingTimer(pub Timer);

pub fn client_ping(
    time: Res<Time>,
    mut timer: ResMut<PingTimer>,
    mut socket: ResMut<ClientSocket>
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let packet_sender = socket.socket.get_packet_sender();
    let bytes;
    if socket.network.connected {
        // Bytes to sent
        bytes = EventMasks::Ping(PingData::Ping).to_bytes();
        socket.network.last_ping = time.time_since_startup().as_millis();
    } else {
        // If the client is not connected, attempt to connect
        bytes = EventMasks::Connection(ConnectionData::Connect).to_bytes();
    }
    let unreliable = Packet::reliable_unordered(socket.server_addr, bytes);
    packet_sender.send(unreliable).unwrap();
}