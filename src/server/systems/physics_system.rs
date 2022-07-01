use bevy::prelude::*;

use crate::{shared::{components::{physics::{PhysicsObject, Collisionable, HitBox, Bouncy}, transform::Transform}}, server::events::physics::CollisionEvent};

pub fn velocity_update(time: Res<Time>, mut query: Query<&mut PhysicsObject>) {
    for mut object in query.iter_mut() {
        let vx_sign = object.vx.signum();
        let vy_sign = object.vy.signum();
        let effective_drag = object.drag * time.delta_seconds_f64();
        let new_vx = vx_sign * (object.vx.abs() - effective_drag);
        let new_vy = vy_sign * (object.vy.abs() - effective_drag);
        object.vx = new_vx;
        object.vy = new_vy;
    }
}

pub fn transform_update(time: Res<Time>, mut query: Query<(&mut Transform, &PhysicsObject)>) {
    let delta = time.delta_seconds_f64();
    for (mut transform, mut physics) in query.iter_mut() {
        let new_x = transform.x + physics.vx * delta;
        let new_y = transform.y + physics.vy * delta;
        transform.x = new_x;
        transform.y = new_y;
    }
}

pub fn collision_update(
    mut query1: Query<(Entity, &mut PhysicsObject, &Collisionable, &HitBox, &Transform)>,
    mut query2: Query<(Entity, &mut PhysicsObject, &Collisionable, &HitBox, &Transform)>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    for (entity, mut physics1, col1, hitbox1, transform1) in query1.iter_mut() {
        // Check against every other physics object with collision
        for (entity, physics2, col2, hitbox2, transform2) in query2.iter_mut() {
            if (col1.collision_mask & col2.collision_mask) == 0 {
                continue;
            }
            // Check if the hitboxes collide
            let rect1 = Rect {
                top: transform1.y - hitbox1.height / 2.0,
                right: transform1.x + hitbox1.width / 2.0,
                bottom: transform1.y + hitbox1.height / 2.0,
                left: transform1.x - hitbox1.width / 2.0,
            };
            let rect2 = Rect {
                top: transform2.y - hitbox2.height / 2.0,
                right: transform2.x + hitbox2.width / 2.0,
                bottom: transform2.y + hitbox2.height / 2.0,
                left: transform2.x - hitbox2.width / 2.0,
            };

            if rect2.top < rect1.top && rect1.top < rect2.bottom {
                collision_events.send(CollisionEvent);
            }
            if rect2.top < rect1.bottom && rect1.bottom < rect2.bottom {
                collision_events.send(CollisionEvent);
            }
            if rect2.left < rect1.left && rect1.left < rect2.right {
                collision_events.send(CollisionEvent);
            }
            if rect2.left < rect1.right && rect1.right < rect2.right {
                collision_events.send(CollisionEvent);
            }
        }
    }
}

pub fn collision_handler(
    mut collision_events: EventReader<CollisionEvent>
) {
    if collision_events.iter().count() > 0 {
        println!("Collision");
    }
}