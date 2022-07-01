use bevy::prelude::*;

use crate::{client::{resources::input_data::InputData, events::input_event::InputEvent}, shared::network::{udp::{ClientSocket, construct_packet}, packet_events::EventMasks}};

pub fn client_input(
    input: Res<Input<KeyCode>>, 
    mut client_input: ResMut<InputData>,
    mut input_events: EventWriter<InputEvent>
) {
    client_input.0.top      = input.pressed(KeyCode::W);
    client_input.0.right    = input.pressed(KeyCode::D);
    client_input.0.bottom   = input.pressed(KeyCode::S);
    client_input.0.left     = input.pressed(KeyCode::A);
    if input.any_just_pressed(vec![KeyCode::W, KeyCode::D, KeyCode::S, KeyCode::A]) {
        input_events.send(InputEvent);
    }
    if input.any_just_released(vec![KeyCode::W, KeyCode::D, KeyCode::S, KeyCode::A]) {
        input_events.send(InputEvent);
    }
}

pub fn client_input_sender(
    socket: ResMut<ClientSocket>,
    client_input: Res<InputData>,
    mut input_events: EventReader<InputEvent>
) {
    if input_events.iter().count() > 0 {
        let packet_sender = socket.socket.get_packet_sender();
        let bytes;
        // Bytes to sent
        bytes = EventMasks::ClientInput(client_input.0).to_bytes();
        let unreliable = construct_packet(bytes, socket.server_addr);
        packet_sender.send(unreliable).unwrap();
    }
}