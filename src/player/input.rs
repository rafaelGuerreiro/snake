use crate::{commons::speed::Speed, AppState};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct UserInput;

#[derive(Default)]
pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(user_input_move));
    }
}

pub fn user_input_move(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<UserInput>>,
) {
    let Some(movement) = key_press_direction(input) else {
        return;
    };

    for (mut transform, speed) in &mut query {
        transform.translation += movement * speed.0 * time.delta_seconds();
    }
}

fn key_press_direction(input: Res<Input<KeyCode>>) -> Option<Vec3> {
    let mut movement = Vec3::default();

    if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement.y += 1f32;
    } else if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement.y -= 1f32;
    }

    if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement.x += 1f32;
    } else if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement.x -= 1f32;
    }

    movement.try_normalize()
}
