use bevy::prelude::*;
use bevy_aseprite::AsepritePlugin;

use game_lib::enemy_spawner::EnemySpawnerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, createCamera)
        .add_plugins(AsepritePlugin)
        .add_plugins(EnemySpawnerPlugin)
        .run();
}

fn createCamera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
