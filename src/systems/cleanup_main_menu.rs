use bevy::prelude::*;
use crate::components::main_menu::MainMenu;

pub fn cleanup_main_menu(
    mut commands: Commands,
    main_menu: Res<MainMenu>
) {
    commands.entity(main_menu.0).despawn();
}