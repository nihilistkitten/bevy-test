//! The graphics system.
use std::io::{Stdout, Write};

use crate::position::Position;
use bevy::prelude::*;

/// The graphics system.
pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(graphics::<Stdout>.system())
            .insert_resource(OutStream(std::io::stdout()));
    }
}

/// An output stream, usually stdout, overwriteable for testing.
struct OutStream<T>(T)
where
    T: Write + bevy::ecs::component::Component;

/// The graphics system.
#[allow(clippy::needless_pass_by_value)] // forced by bevy apis
fn graphics<T>(mut w: ResMut<OutStream<T>>, query: Query<&Position>)
where
    T: Write + bevy::ecs::component::Component,
{
    for position in query.iter() {
        // safety: in production this is stdout
        write!(w.0, "{}", position).expect("buffer out of memory");
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn graphics_works() {
        let mut world = World::default();
        world.spawn().insert(Position { x: 0.0, y: 0.0 }).id();

        let mut update_stage = SystemStage::parallel();

        update_stage.add_system(graphics::<Cursor<Vec<u8>>>.system());

        let buff = Cursor::new(vec![0; 6]); // output should be 6 bytes long
        world.insert_resource(OutStream(buff));

        update_stage.run(&mut world);

        assert_eq!(
            (*world
                .get_resource::<OutStream<Cursor<Vec<u8>>>>()
                .expect("the world has an outstream"))
            .0
            .get_ref(),
            b"(0, 0)"
        );
    }
}
