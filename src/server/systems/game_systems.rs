use bevy::prelude::*;

use crate::{server::components::server_bundles::ThonkBundle, shared::components::{transform::Transform, thonk::Thonk, owned::PlayerOwned}};

pub fn setup(mut commands: Commands) {
    commands
    .spawn_bundle(ThonkBundle {
        transform: Transform {
            x: 10.0,
            y: 0.0,
        },
        thonk: Thonk {
            width: 2.0,
            height: 5.0,
            speed: 1.0,
        },
        player: PlayerOwned {
            handle: 0
        }
    });
    commands.spawn_bundle(ThonkBundle {
        transform: Transform {
            x: 40.0,
            y: 0.0,
        },
        thonk: Thonk {
            width: 2.0,
            height: 5.0,
            speed: 1.0,
        },
        player: PlayerOwned {
            handle: 1
        }
    });
}