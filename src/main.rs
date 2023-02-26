use crate::player::{create_player, input::user_input_move};
use bevy::prelude::*;

pub mod commons;
pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(user_input_move)
        .add_startup_system(setup)
        .add_startup_system(create_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
