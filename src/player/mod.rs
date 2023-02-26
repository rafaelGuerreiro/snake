use crate::{
    commons::speed::Speed,
    player::input::{UserInput, UserInputPlugin},
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub mod input;

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UserInputPlugin)
            .add_system_to_stage(CoreStage::PostUpdate, camera_follow_player)
            .add_startup_system(create_player);
    }
}

pub fn create_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
            transform: Transform::from_translation(Vec3::default()),
            ..default()
        })
        .id();

    commands
        .spawn((
            Player::default(),
            TransformBundle::default(),
            VisibilityBundle::default(),
            UserInput::default(),
            Speed(300f32),
        ))
        .add_child(entity);
}

pub fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let mut position = player.single().translation;
    position.z = 1000f32;
    camera.single_mut().translation = position;
}
