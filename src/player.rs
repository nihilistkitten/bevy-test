//! The player system.
use bevy::prelude::*;

use crate::movement::Movable;

/// The player system.
///
/// Adds the player on startup.
pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_player_materials.system())
            // use a stage because spawn_player depends on the material existing
            .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()));
    }
}

/// Create the materials the player depends on.
#[allow(clippy::needless_pass_by_value)] // required by bevy api
fn create_player_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(112.0, 75.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(Materials {
        player_material: texture_atlas_handle,
    });
}

/// Add the player to the world.
#[allow(clippy::needless_pass_by_value)] // required by bevy API
fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: materials.player_material.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..SpriteSheetBundle::default()
        })
        .insert_bundle(Movable::default())
        .insert(Player);
}

struct Player;

struct Materials {
    player_material: Handle<TextureAtlas>,
}

/*
#[cfg(test)]
mod tests {
    use super::*;
} */
