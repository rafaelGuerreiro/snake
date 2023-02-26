use crate::{commons::speed::Speed, player::input::UserInput};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub mod input;

pub fn create_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        })
        .id();

    commands
        .spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            UserInput::default(),
            Speed(300f32),
        ))
        .add_child(entity);
}
