use std::time::Instant;

use bevy::{prelude::{ResMut, Res}, core::Time};
use laminar::{SocketEvent, Packet};

use crate::shared::network::{udp::ClientSocket, packet_events::{EventMasks, PingData, ConnectionData}};

pub fn client_update(
    mut client_socket: ResMut<ClientSocket>,
    time: Res<Time>
) {
    let packet_send_count = client_socket.socket.get_packet_sender().len();
    let mut net = client_socket.network;
    net.packets.sent += packet_send_count as u32;
    if packet_send_count > 0 {
        net.last_packets.sent.0 = net.last_packets.sent.1;
        net.last_packets.sent.1 = time.time_since_startup().as_millis();
        net.rates.sent = 1000.0 / (net.last_packets.sent.1 - net.last_packets.sent.0) as f64
    }
    client_socket.socket.manual_poll(Instant::now());
    client_socket.network = net;
    if let Some(packet) = client_socket.socket.recv() {
        match packet {
            SocketEvent::Packet(packet) => {
                net.packets.recieved += 1;
                net.last_packets.recieved.0 = net.last_packets.recieved.1;
                net.last_packets.recieved.1 = time.time_since_startup().as_millis();
                net.rates.recieved = 1000.0 / (net.last_packets.recieved.1 - net.last_packets.recieved.0) as f64;
                client_socket.network = net;
                packet_handler(&mut client_socket, packet, &time)
            },
            SocketEvent::Connect(_) => (),
            SocketEvent::Timeout(_) => {
                // Timeout?
                net.connected = false;
            },
            SocketEvent::Disconnect(_) => (),
        }
    }
}

pub fn packet_handler(
    client_socket: &mut ClientSocket, 
    packet: Packet,
    time: &Time
) {
    let payload = packet.payload();
    let event = EventMasks::from_bytes(payload).unwrap();
    match event {
        EventMasks::Ping(ping_data) => {
            ping_handler(client_socket, ping_data, &time)
        },
        EventMasks::Connection(event_data) => {
            connection_handler(client_socket, event_data);
        }
        _ => ()
    }
}

pub fn ping_handler(
    client_socket: &mut ClientSocket, 
    ping_data: PingData,
    time: &Time
) {
    match ping_data {
        PingData::Pong => {
            client_socket.network.last_pong = time.time_since_startup().as_millis();
        },
        _ => ()
    }
}

pub fn connection_handler(
    client_socket: &mut ClientSocket, 
    event_data: ConnectionData
) {
    match event_data {
        ConnectionData::Ack => {
            println!("Server Ack Recieved");
            client_socket.network.connected = true;
        },
        _ => ()
    }
}