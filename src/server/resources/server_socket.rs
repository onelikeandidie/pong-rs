use std::net::SocketAddr;

use laminar::Socket;

use super::client_data::ClientList;

pub struct ServerSocket { 
    pub socket: Socket,
    pub clients: Vec<SocketAddr>,
    pub client_data: ClientList
}

impl ServerSocket {
    pub fn add_client(&mut self, socket_addr: SocketAddr) {
        self.client_data.new_client_input(None);
        self.clients.push(socket_addr);
        println!("New Client Connected: {}", socket_addr);
    }

    pub fn forget_client(&mut self, socket_addr: SocketAddr) -> Result<(), ()> {
        if let Some(client_index) = self.clients.iter().position(|addr| {
            addr.to_string() == socket_addr.to_string()
        }) {
            self.clients.remove(client_index);
            self.client_data.clear_client_input(client_index).unwrap();
            println!("Client Disconnected: {}", socket_addr);
            return Ok(())
        }
        Err(())
    }

    pub fn is_connected(&self, socket_addr: SocketAddr) -> bool {
        if let Some(_) = self.clients.iter().position(|addr| {
            addr.to_string() == socket_addr.to_string()
        }) {
            return true;
        }
        false
    }

    pub fn get_client_index(&self, socket_addr: SocketAddr) -> Result<usize, ()> {
        if let Some(index) = self.clients.iter().position(|addr| {
            addr.to_string() == socket_addr.to_string()
        }) {
            return Ok(index);
        }
        Err(())
    }
}