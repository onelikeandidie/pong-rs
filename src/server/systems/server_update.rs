use std::time::Instant;

use bevy::prelude::ResMut;
use laminar::{SocketEvent, Packet};

use crate::{shared::network::{packet_events::{EventMasks, PingData, ConnectionData, ClientInputData}}, server::resources::{server_socket::ServerSocket}};

pub fn server_update(mut server_socket: ResMut<ServerSocket>) {
    server_socket.socket.manual_poll(Instant::now());
    if let Some(packet) = server_socket.socket.recv() {
        match packet {
            SocketEvent::Packet(packet) => {
                packet_handler(&mut server_socket, packet);
            },
            SocketEvent::Connect(_) => (),
            SocketEvent::Timeout(_) => (),
            SocketEvent::Disconnect(addr) => {
                if let Ok(_) = server_socket.forget_client(addr) {}
            },
        }
    }
}

fn packet_handler(socket: &mut ServerSocket, packet: Packet) {
    let payload = packet.payload();
    let event = EventMasks::from_bytes(payload).unwrap();
    match event {
        EventMasks::Ping(event_data) => packet_ping_handler(&socket, packet, event_data),
        EventMasks::Connection(event_data) => packet_connection_handler(socket, packet, event_data),
        EventMasks::ClientInput(event_data) => packet_client_input_handler(socket, packet, event_data),
    }
}

fn packet_ping_handler(socket: &ServerSocket, packet: Packet, event_data: PingData) {
    match event_data {
        PingData::Ping => {
            let packet_sender = socket.socket.get_packet_sender();
            let bytes = EventMasks::Ping(PingData::Pong).to_bytes();
            let unreliable = Packet::reliable_unordered(packet.addr(), bytes);
            packet_sender.send(unreliable).unwrap();
        },
        PingData::Pong => (),
    }
}

fn packet_connection_handler(socket: &mut ServerSocket, packet: Packet, event_data: ConnectionData) {
    let packet_sender = socket.socket.get_packet_sender();
    let bytes;
    let addr = packet.addr();
    match event_data {
        ConnectionData::Connect => {
            socket.add_client(addr);
            bytes = EventMasks::Connection(ConnectionData::Ack).to_bytes();
        },
        ConnectionData::Disconnect => {
            if let Ok(_) = socket.forget_client(addr) {
                bytes = EventMasks::Connection(ConnectionData::Ack).to_bytes();
            } else {
                return;
            }
        },
        ConnectionData::Ack => return,
    }
    println!("Recieved: {:?}", event_data);
    let unreliable = Packet::reliable_unordered(addr, bytes);
    packet_sender.send(unreliable).unwrap();
}

fn packet_client_input_handler(socket: &mut ServerSocket, packet: Packet, event_data: ClientInputData) {
    let index = socket.get_client_index(packet.addr());
    match index {
        Ok(handle) => {
            if let Ok(_) = socket.client_data.update_client_input(handle, event_data) {
                
            }
        },
        Err(_) => {},
    }
}