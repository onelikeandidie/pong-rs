pub mod components {
    pub mod ball;
    pub mod owned;
    pub mod physics;
    pub mod player;
    pub mod thonk;
    pub mod transform;
}

pub mod network {
    pub mod packet_events;
    pub mod server_state;
    pub mod udp;
}

pub mod plugins {
    pub mod cli_plugin;
}

pub mod systems {
    
}

pub mod resources {
    pub mod cli_args;
}

pub mod util;