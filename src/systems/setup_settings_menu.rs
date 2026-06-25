use bevy::prelude::*;
use crate::{
    observers::menu::{
        on_rotary_encoder_button_pushed::on_rotary_encoder_button_pushed,
        on_rotary_encoder_rotated::on_rotary_encoder_rotated
    },
    components::{
        menu::Menu,
        settings_menu::SettingsMenu,
        menu_items::MenuItems,
        menu_item::{
            MenuItem,
            SelectedItem
        },
        menu_icon::MenuIcon
    },
    plugins::settings_menu::SettingsMenuState,
    resources::navigation_stack::NavigationStack
};
use tinytga::Tga;

pub fn setup_settings_menu(
    mut commands: Commands,
    mut navigation_stack: ResMut<NavigationStack>
) {
    navigation_stack.push(
        commands.spawn((
            (
                Menu,
                SettingsMenu,
                Name::new("Settings Menu")
            ),
            related!(MenuItems[
                (MenuItem(SettingsMenuState::One), Name::new("One"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap()), SelectedItem),
                (MenuItem(SettingsMenuState::Two), Name::new("Two"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(SettingsMenuState::Three), Name::new("Three"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(SettingsMenuState::Four), Name::new("Four"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(SettingsMenuState::Five), Name::new("Five"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap())),
                (MenuItem(SettingsMenuState::Back), Name::new("Back"), MenuIcon(Tga::from_slice(include_bytes!("../../resources/gears.tga")).unwrap()))
            ]),
        ))
            .observe(on_rotary_encoder_button_pushed::<SettingsMenuState>)
            .observe(on_rotary_encoder_rotated::<SettingsMenu, SettingsMenuState>)
            .id()
    )
}