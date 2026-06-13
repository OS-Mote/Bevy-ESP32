use bevy::prelude::*;
use crate::systems::get_input::get_input;
use crate::{app_state::AppState, systems::process_main_menu_input::process_main_menu_input};
use crate::systems::clear_frame_buffer::clear_frame_buffer;
use crate::systems::draw_main_menu::draw_main_menu;
use crate::systems::main_menu_setup::main_menu_setup;
use enum_iterator::Sequence;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Sequence)]
pub enum MainMenuState {
    #[default]
    Foo,
    Bar,
    Baz
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .add_systems(Update, process_main_menu_input.after(get_input).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, draw_main_menu.after(clear_frame_buffer).run_if(in_state(AppState::MainMenu)))
        .init_state::<MainMenuState>();
}