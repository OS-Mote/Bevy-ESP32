use bevy::prelude::*;
use crate::{
    observers::menu::{
        on_rotary_encoder_button_pushed::on_rotary_encoder_button_pushed,
        on_rotary_encoder_rotated::on_rotary_encoder_rotated
    },
    components::{
        menu::Menu,
        main_menu::MainMenu,
        menu_items::MenuItems,
        menu_item::{
            MenuItem,
            SelectedItem
        },
        menu_icon::MenuIcon
    },
    plugins::main_menu::MainMenuState,
    resources::navigation_stack::NavigationStack
};
use tinytga::Tga;

pub fn setup_main_menu(
    mut commands: Commands,
    mut navigation_stack: ResMut<NavigationStack>
) {
    navigation_stack.push(
        commands.spawn((
            (
                Menu,
                MainMenu,
                Name::new("Main Menu")
            ),
            related!(MenuItems[
                (MenuItem(MainMenuState::Foo), Name::new("Foo"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap()), SelectedItem),
                (MenuItem(MainMenuState::Bar), Name::new("Bar"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(MainMenuState::Baz), Name::new("Baz"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(MainMenuState::Quux), Name::new("Quux"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(MainMenuState::Settings), Name::new("Settings"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap()))
            ])
        ))
            .observe(on_rotary_encoder_button_pushed::<MainMenuState>)
            .observe(on_rotary_encoder_rotated::<MainMenu, MainMenuState>)
            .id()
    )
}




