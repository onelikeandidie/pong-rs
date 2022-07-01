use bevy::prelude::Component;

#[derive(Component)]
pub struct PingDisplay {
    pub ping: u128
}

#[derive(Component)]
pub struct ConnectionDisplay;

#[derive(Component)]
pub struct PacketsDisplay;