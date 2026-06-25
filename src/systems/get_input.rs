use bevy::prelude::*;
use esp_println::print;
use esp_println::println;

use crate::events::rotary_encoder_button_pressed_event::RotaryEncoderButtonPressedEvent;
use crate::events::rotary_encoder_rotated_event::RotaryEncoderRotatedEvent;
use crate::resources::navigation_stack;
use crate::resources::navigation_stack::NavigationStack;
use crate::rotary_encoder::RotaryEncoderDirection;
use crate::rotary_encoder::ROTARY_ENCODER_DIRECTION;
use crate::rotary_encoder::ROTARY_ENCODER_BUTTON_PRESS;

pub fn get_input(
    mut commands: Commands,
    mut navigation_stack: ResMut<NavigationStack>
) {
    if let Some(top_navigation_entity) = navigation_stack.last() {
        critical_section::with(|cs| {
            for direction in ROTARY_ENCODER_DIRECTION.borrow_ref(cs).iter() {
                match direction {
                    RotaryEncoderDirection::Clockwise => {
                        commands.trigger(RotaryEncoderRotatedEvent{ entity: *top_navigation_entity, direction: RotaryEncoderDirection::Clockwise });
                    }

                    RotaryEncoderDirection::Anticlockwise => {
                        commands.trigger(RotaryEncoderRotatedEvent{ entity: *top_navigation_entity, direction: RotaryEncoderDirection::Anticlockwise });
                    }
                }
            }

            ROTARY_ENCODER_DIRECTION.borrow_ref_mut(cs).clear();

            for _ in ROTARY_ENCODER_BUTTON_PRESS.borrow_ref(cs).iter() {
                commands.trigger(RotaryEncoderButtonPressedEvent{ entity: *top_navigation_entity });
            }

            ROTARY_ENCODER_BUTTON_PRESS.borrow_ref_mut(cs).clear();
        });
    }
}