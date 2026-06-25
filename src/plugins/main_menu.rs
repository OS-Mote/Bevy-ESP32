use bevy::prelude::*;
use crate::components::main_menu::MainMenu;
use crate::systems::cleanup_main_menu::cleanup_main_menu;
use crate::systems::get_input::get_input;
use crate::systems::update_display::update_display;
use crate::{app_state::AppState, systems::process_menu_input::process_menu_input};
use crate::systems::clear_frame_buffer::clear_frame_buffer;
use crate::systems::draw_menu::draw_menu;
use crate::systems::setup_main_menu::setup_main_menu;
use enum_iterator::{
    Sequence, all, next, previous
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates, Sequence)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    None,
    Foo,
    Bar,
    Baz,
    Quux,
    Settings
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(Update, draw_menu::<MainMenu, MainMenuState>.after(get_input).before(update_display).run_if(in_state(MainMenuState::None)))
        .add_sub_state::<MainMenuState>();
}