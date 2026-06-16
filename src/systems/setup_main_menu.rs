use bevy::prelude::*;
use crate::{
    components::{
        main_menu::MainMenu, menu::{
            Menu,
            SelectedMenu
        },
        menu_item::{
            MenuItem,
            SelectedItem
        },
        menu_items::MenuItems
    },
    plugins::main_menu::MainMenuState
};

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        (
            Menu,
            Name::new("Main Menu"),
            SelectedMenu
        ),
        related!(MenuItems[
            (MenuItem(MainMenuState::Foo), Name::new("Foo"), SelectedItem),
            (MenuItem(MainMenuState::Bar), Name::new("Bar")),
            (MenuItem(MainMenuState::Baz), Name::new("Baz")),
            (MenuItem(MainMenuState::Quux), Name::new("Quux")),
            (MenuItem(MainMenuState::Settings), Name::new("Settings"))
        ])
    ));
}