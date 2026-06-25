use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct RotaryEncoderButtonPressedEvent {
    pub entity: Entity
}