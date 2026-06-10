use bevy::prelude::*;
use crate::rotary_encoder::RotaryEncoderDirection;
use crate::messages::rotary_encoder_moved::RotaryEncoderMovedMessage;

pub fn process_input (
    mut message_reader: MessageReader<RotaryEncoderMovedMessage>,
    mut transforms: Query<&mut Transform>,
) {
    for message in message_reader.read() {
        for mut transform in &mut transforms {
            let direction = transform.local_x();

            match message.0 {
                RotaryEncoderDirection::Clockwise => {
                    transform.translation += direction * 8.0;
                },
                RotaryEncoderDirection::Anticlockwise => {
                    transform.translation += direction * -8.0;
                },
            }
        }
    }
}
