use bevy::prelude::*;
use crate::rotary_encoder::RotaryEncoderDirection;

#[derive(EntityEvent)]
pub struct RotaryEncoderRotatedEvent {
    pub entity: Entity,
    pub direction: RotaryEncoderDirection
}