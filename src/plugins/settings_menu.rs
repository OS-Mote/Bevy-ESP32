use bevy::prelude::*;
use crate::plugins::main_menu::MainMenuState;
use crate::systems::get_input::get_input;
use crate::{app_state::AppState, systems::process_menu_input::process_menu_input};
use crate::systems::clear_frame_buffer::clear_frame_buffer;
use crate::systems::draw_menu::draw_menu;
use crate::systems::setup_main_menu::setup_main_menu;
use enum_iterator::Sequence;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates, Sequence)]
#[source(MainMenuState = MainMenuState::Settings)]
pub enum SettingsMenuState {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

pub fn settings_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MainMenuState::Settings), setup_main_menu)
        .add_systems(Update, process_menu_input::<SettingsMenuState>.after(get_input).run_if(in_state(MainMenuState::Settings)))
        .add_systems(Update, draw_menu::<SettingsMenuState>.after(clear_frame_buffer).run_if(in_state(MainMenuState::Settings)))
        .add_sub_state::<SettingsMenuState>();
}