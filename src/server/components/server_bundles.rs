use bevy::prelude::Bundle;

use crate::shared::components::{transform::Transform, thonk::Thonk, owned::PlayerOwned};

#[derive(Bundle)]
pub struct ThonkBundle {
    pub transform: Transform,
    pub thonk: Thonk,
    pub player: PlayerOwned
}