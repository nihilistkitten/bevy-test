pub mod movement;
pub mod player;
pub mod position;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system())
        .add_plugin(player::System)
        .add_plugin(movement::System)
        .run();
}

/// Add the camera to the world.
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
