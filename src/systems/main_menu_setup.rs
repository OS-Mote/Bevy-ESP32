use bevy::prelude::*;
use esp_println::{print, println};
use crate::components::{menu::Menu, menu_item::MenuItem};

pub fn main_menu_setup(mut commands: Commands) {
    commands.spawn((
        Menu(String::from("Main Menu")),
        children![
            MenuItem(String::from("First")),
            MenuItem(String::from("Second"))
        ]
    ));
}