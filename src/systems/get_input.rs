use bevy::prelude::*;

use crate::rotary_encoder::RotaryEncoderDirection;
use crate::rotary_encoder::ROTARY_ENCODER_DIRECTION;
use crate::messages::rotary_encoder_moved::RotaryEncoderMovedMessage;

pub fn get_input(
    mut message_writer: MessageWriter<RotaryEncoderMovedMessage>
) {
    critical_section::with(|cs| {
        for direction in ROTARY_ENCODER_DIRECTION.borrow_ref(cs).iter() {
            match direction {
                RotaryEncoderDirection::Clockwise => {
                    message_writer.write(RotaryEncoderMovedMessage(
                        RotaryEncoderDirection::Clockwise
                    ));
                }

                RotaryEncoderDirection::Anticlockwise => {
                    message_writer.write(RotaryEncoderMovedMessage(
                        RotaryEncoderDirection::Anticlockwise
                    ));
                }
            }
        }

        ROTARY_ENCODER_DIRECTION.borrow_ref_mut(cs).clear();
    });
}