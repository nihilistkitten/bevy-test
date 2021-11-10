pub mod graphics;
pub mod movement;
pub mod position;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player.system())
        .add_startup_system(spawn_camera.system())
        .add_plugin(movement::System)
        .add_plugin(graphics::System)
        .run();
}

/// Add the camera to the world.
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Add the player to the world.
fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(movement::Movable::new(0.0, 0.0, 1.0, -1.0));
}
