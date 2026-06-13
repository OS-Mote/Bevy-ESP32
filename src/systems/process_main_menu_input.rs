use bevy::prelude::*;
use crate::plugins::main_menu::MainMenuState;
use crate::rotary_encoder::RotaryEncoderDirection;
use crate::messages::rotary_encoder_moved::RotaryEncoderMovedMessage;
use enum_iterator::{
    all,
    next,
    previous
};

pub fn process_main_menu_input (
    mut message_reader: MessageReader<RotaryEncoderMovedMessage>,
    main_menu_state: Res<State<MainMenuState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>
) {
    for message in message_reader.read() {
        match message.0 {
            RotaryEncoderDirection::Clockwise => {
                if let Some(next_state) = next(&**main_menu_state) {
                    next_main_menu_state.set(next_state);    
                }   
            },
            RotaryEncoderDirection::Anticlockwise => {
                if let Some(previous_state) = previous(&**main_menu_state) {
                    next_main_menu_state.set(previous_state);    
                }   
            }
        }
    }
}