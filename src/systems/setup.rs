use bevy::{
    prelude::{
        *,
        Transform
    }
};
// use crate::components::menu::{
//     Menu,
//     Item
// };

pub fn setup(
    mut commands: Commands
) {
    commands.spawn(Transform::from_xyz(0.0, 0.0, 1.0));

    // commands.spawn(
    //     Menu(
    //         vec![

    //             Item(String::from("Foo")),
    //             Item(String::from("Bar"))
    //         ]
    //     )
    // );
}
