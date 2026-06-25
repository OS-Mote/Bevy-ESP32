use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::{components::menu_item::{MenuItem, SelectedItem}, events::rotary_encoder_button_pressed_event::RotaryEncoderButtonPressedEvent};
use enum_iterator::Sequence;

pub fn on_rotary_encoder_button_pushed<T: States + FreelyMutableState + Sequence + Copy>(
    event: On<RotaryEncoderButtonPressedEvent>,
    mut next_menu_state: ResMut<NextState<T>>,
    selected_menu_item_query: Single<&MenuItem<T>, With<SelectedItem>>
) {
    let selected_menu_item = selected_menu_item_query.into_inner();

    next_menu_state.set(selected_menu_item.0);
}