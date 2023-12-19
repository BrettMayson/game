use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Query, Res, Resource},
    },
    sprite::SpriteBundle,
    time::{Time, Timer, TimerMode},
    transform::{commands, components::Transform},
    utils::info,
};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

use self::hobbit::Hobbit;

mod hobbit;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update)
            .add_systems(Update, kill)
            .add_systems(Update, has_died);
    }
}

#[derive(Resource)]
pub struct EnemyManager {
    enemies: Vec<Entity>,
    timer: Timer,
}

fn startup(mut commands: bevy::prelude::Commands) {
    commands.insert_resource(EnemyManager {
        enemies: Vec::new(),
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });
}

fn update(
    time: Res<Time>,
    mut commands: bevy::prelude::Commands,
    mut enemy_manager: bevy::prelude::ResMut<EnemyManager>,
    asset_server: Res<AssetServer>,
) {
    enemy_manager.timer.tick(time.delta());

    if enemy_manager.timer.finished() {
        enemy_manager.timer.reset();

        let enemy = commands.spawn((
            Hobbit,
            AsepriteBundle {
                aseprite: asset_server.load("hobbit/export.aseprite"),
                animation: AsepriteAnimation::from("idle"),
                transform: Transform {
                    // random pos
                    translation: bevy::math::Vec3::new(
                        (rand::random::<f32>() * 100.0 - 50.0) * 5.0,
                        (rand::random::<f32>() * 100.0 - 50.0) * 5.0,
                        0.0,
                    ),
                    scale: bevy::math::Vec3::new(5.0, 5.0, 5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        enemy_manager.enemies.push(enemy.id());

        if enemy_manager.enemies.len() > 3 {
            let enemy = enemy_manager.enemies.remove(0);
            // add Ddying component
            commands.entity(enemy).insert(ShouldKill);
        }
    }
}

#[derive(Component)]
pub struct ShouldKill;

fn kill(
    mut commands: bevy::prelude::Commands,
    mut query: Query<(&mut AsepriteAnimation, Entity), (With<Hobbit>, With<ShouldKill>)>,
) {
    for (mut animation, entity) in query.iter_mut() {
        *animation = AsepriteAnimation::from("death");

        commands.entity(entity).insert(Dying);
        commands.entity(entity).remove::<ShouldKill>();
    }
}

#[derive(Component)]
pub struct Dying;

fn has_died(
    mut commands: bevy::prelude::Commands,
    mut query: Query<(&AsepriteAnimation, Entity), (With<Hobbit>, With<Dying>)>,
) {
    for (animation, entity) in query.iter_mut() {
        if !animation.is_playing() || animation.current_frame >= 49 {
            commands.entity(entity).despawn();
        }
    }
}
