use bevy::prelude::*;
use crate::components::settings_menu::SettingsMenu;
use crate::plugins::main_menu::MainMenuState;
use crate::systems::cleanup_settings_menu;
use crate::systems::get_input::get_input;
use crate::systems::setup_settings_menu::setup_settings_menu;
use crate::systems::cleanup_settings_menu::cleanup_settings_menu;
use crate::{app_state::AppState, systems::process_menu_input::process_menu_input};
use crate::systems::clear_frame_buffer::clear_frame_buffer;
use crate::systems::draw_menu::draw_menu;
use crate::systems::setup_main_menu::setup_main_menu;
use enum_iterator::Sequence;
use crate::update_display;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates, Sequence)]
#[source(MainMenuState = MainMenuState::Settings)]
pub enum SettingsMenuState {
    #[default]
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Back
}

pub fn settings_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MainMenuState::Settings), setup_settings_menu)
        .add_systems(OnEnter(SettingsMenuState::Back), cleanup_settings_menu)
        .add_systems(Update, draw_menu::<SettingsMenu, SettingsMenuState>.after(get_input).before(update_display).run_if(in_state(MainMenuState::Settings)))
        .add_sub_state::<SettingsMenuState>();
}