use bevy::prelude::*;

mod game_plugin;
mod menu_plugin;
mod splash_plugin;
mod math;
mod noise;

use game_plugin::GamePlugin;
use menu_plugin::MenuPlugin;
use splash_plugin::SplashPlugin;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Loading,
    Game
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u8);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Volume(7))
    .add_startup_system(setup)
    .add_state(GameState::Splash)
    .add_plugin(SplashPlugin)
    .add_plugin(MenuPlugin)
    .add_plugin(GamePlugin)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}