//! The movement system.
use crate::position::Position;
use bevy::prelude::*;

/// The movement system.
pub struct System;

/// An entity subject to the movement system.
#[derive(Bundle)]
pub struct Movable {
    position: Position,
    velocity: Velocity,
}

impl Movable {
    #[must_use]
    /// Create a new Movable bundle.
    pub const fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        let position = Position { x, y };
        let velocity = Velocity { x: vx, y: vy };
        Self { position, velocity }
    }
}

impl Plugin for System {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system());
    }
}

/// A velocity component.
struct Velocity {
    x: f32,
    y: f32,
}

/// The movement system.
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.update(velocity);
    }
}

impl Position {
    /// Move the position one tick.
    fn update(&mut self, v: &Velocity) {
        self.x += v.x;
        self.y += v.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_works() {
        let mut world = World::default();
        let entity = world
            .spawn()
            .insert_bundle(Movable::new(0.0, 0.0, 1.0, -1.0))
            .id();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(movement.system());

        update_stage.run(&mut world);

        assert_eq!(
            world
                .entity(entity)
                .get::<Position>()
                .expect("the world should have an entity with position"),
            &Position { x: 1.0, y: -1.0 }
        );
    }
}
