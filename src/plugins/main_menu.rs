use bevy::prelude::*;
use crate::app_state::AppState;
use crate::systems::main_menu_setup::main_menu_setup;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MainMenuState {
    #[default]
    First,
    Second
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .init_state::<MainMenuState>();
        
}