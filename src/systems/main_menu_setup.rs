use bevy::prelude::*;
use crate::{components::{
    menu::Menu,
    menu_item::MenuItem,
    menu_items::MenuItems
}, plugins::main_menu::MainMenuState};

pub fn main_menu_setup(mut commands: Commands) {
    commands.spawn((
        (Menu, Name::new("Main Menu")),
        related!(MenuItems[
            (MenuItem(MainMenuState::Foo), Name::new("Foo")),
            (MenuItem(MainMenuState::Bar), Name::new("Bar")),
            (MenuItem(MainMenuState::Baz), Name::new("Baz"))
        ])
    ));
}