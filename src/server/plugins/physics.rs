use bevy::prelude::*;

use crate::server::{systems::physics_system::{velocity_update, transform_update, collision_handler, collision_update}, events::physics::CollisionEvent};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_system(velocity_update)
            .add_system(transform_update)
            // .add_system(collision_update)
            .add_system(collision_handler);
    }
}