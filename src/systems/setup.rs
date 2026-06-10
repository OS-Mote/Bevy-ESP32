use bevy::{
    prelude::{
        *,
        Transform
    }
};

pub fn setup(mut commands: Commands) {
    commands.spawn(Transform::from_xyz(0.0, 0.0, 1.0));
}
