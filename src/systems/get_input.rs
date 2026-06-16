use bevy::prelude::*;

use crate::rotary_encoder::RotaryEncoderDirection;
use crate::rotary_encoder::ROTARY_ENCODER_DIRECTION;
use crate::rotary_encoder::ROTARY_ENCODER_BUTTON_PRESS;
use crate::messages::rotary_encoder_moved::RotaryEncoderMovedMessage;
use crate::messages::rotary_encoder_button_pressed::RotaryEncoderButtonPressedMessage;

pub fn get_input(
    mut rotary_encoder_moved_message_writer: MessageWriter<RotaryEncoderMovedMessage>,
    mut rotary_encoder_button_pressed_message_writer: MessageWriter<RotaryEncoderButtonPressedMessage>
) {
    critical_section::with(|cs| {
        for direction in ROTARY_ENCODER_DIRECTION.borrow_ref(cs).iter() {
            match direction {
                RotaryEncoderDirection::Clockwise => {
                    rotary_encoder_moved_message_writer.write(RotaryEncoderMovedMessage(
                        RotaryEncoderDirection::Clockwise
                    ));
                }

                RotaryEncoderDirection::Anticlockwise => {
                    rotary_encoder_moved_message_writer.write(RotaryEncoderMovedMessage(
                        RotaryEncoderDirection::Anticlockwise
                    ));
                }
            }
        }

        ROTARY_ENCODER_DIRECTION.borrow_ref_mut(cs).clear();

        for press in ROTARY_ENCODER_BUTTON_PRESS.borrow_ref(cs).iter() {
            rotary_encoder_button_pressed_message_writer.write(RotaryEncoderButtonPressedMessage);
        }

        ROTARY_ENCODER_BUTTON_PRESS.borrow_ref_mut(cs).clear();
    });
}