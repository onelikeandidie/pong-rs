use std::net::SocketAddr;
use laminar::{Socket, Packet};

/// The socket address of where the server is located.
const SERVER_ADDR: &str = "127.0.0.1:27808";
// The client address from where the data is sent.
const CLIENT_ADDR: &str = "127.0.0.1:27809";

pub fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}

pub fn client_address() -> SocketAddr {
    CLIENT_ADDR.parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
pub struct PacketsTuple<T> {
    pub sent: T,
    pub recieved: T
}

impl Default for PacketsTuple<f64> {
    fn default() -> Self {
        Self { sent: 0.0, recieved: 0.0 }
    }
}
impl Default for PacketsTuple<u32> {
    fn default() -> Self {
        Self { sent: 0, recieved: 0 }
    }
}
impl Default for PacketsTuple<(u128, u128)> {
    fn default() -> Self {
        Self { sent: (0,0), recieved: (0,0) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NetworkStats {
    pub connected: bool,
    pub last_ping: u128,
    pub last_pong: u128,
    pub packets: PacketsTuple<u32>,
    pub last_packets: PacketsTuple<(u128, u128)>,
    pub rates: PacketsTuple<f64>
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self { 
            last_ping: 0, 
            last_pong: 0, 
            connected: false, 
            packets: Default::default(),
            last_packets: Default::default(),
            rates: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ClientSocket { 
    pub socket: Socket, 
    pub server_addr: SocketAddr,
    pub network: NetworkStats,
}

pub fn construct_packet(bytes: Vec<u8>, addr: SocketAddr) -> laminar::Packet {
    Packet::reliable_unordered(addr, bytes)
}