use bevy::prelude::*;

use crate::{server::resources::server_socket::ServerSocket, shared::components::{physics::PhysicsObject, owned::PlayerOwned, thonk::Thonk}};

pub fn update_player_thonks(
    time: Res<Time>,
    socket: Res<ServerSocket>, 
    mut query: Query<(&Thonk, &mut PhysicsObject, &PlayerOwned)>,
) {
    let delta = time.delta().as_secs_f64();
    for (thonk, mut physics, ownership) in query.iter_mut() {
        let client_data = 
            socket.client_data.get(ownership.handle as usize);
        
        if let Some(client_data) = client_data {
            if client_data.input.top {
                physics.vy -= thonk.speed * delta;
            }
            if client_data.input.right {
                physics.vx += thonk.speed * delta;
            }
            if client_data.input.bottom {
                physics.vy += thonk.speed * delta;
            }
            if client_data.input.left {
                physics.vx -= thonk.speed * delta;
            }
        }
    }
}