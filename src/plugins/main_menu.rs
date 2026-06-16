use bevy::prelude::*;
use crate::systems::cleanup_main_menu::cleanup_main_menu;
use crate::systems::get_input::get_input;
use crate::{app_state::AppState, systems::process_menu_input::process_menu_input};
use crate::systems::clear_frame_buffer::clear_frame_buffer;
use crate::systems::draw_menu::draw_menu;
use crate::systems::setup_main_menu::setup_main_menu;
use enum_iterator::Sequence;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates, Sequence)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    Foo,
    Bar,
    Baz,
    Quux,
    Settings,
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        // .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        .add_systems(Update, process_menu_input::<MainMenuState>.after(get_input).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, draw_menu::<MainMenuState>.after(clear_frame_buffer).run_if(in_state(AppState::MainMenu)))
        .add_sub_state::<MainMenuState>();
}