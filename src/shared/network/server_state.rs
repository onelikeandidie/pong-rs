use crate::shared::components::{transform::Transform, thonk::Thonk, owned::PlayerOwned, ball::Ball};


#[derive(Debug, Default)]
pub struct ServerState {
    pub player_list: Vec<PlayerEntity>,
    pub ball_list: Vec<BallEntity>
}

#[derive(Debug, Default)]
pub struct PlayerEntity {
    transform: Transform,
    thonk: Thonk,
    owned: PlayerOwned
}

#[derive(Debug, Default)]
pub struct BallEntity {
    transform: Transform,
    thonk: Ball
}
