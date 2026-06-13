use bevy::prelude::*;

use crate::plugins::main_menu::MainMenuState;

#[derive(Component)]
pub struct MenuItem(pub MainMenuState);