use bevy::prelude::*;
use crate::rotary_encoder::RotaryEncoderDirection;

#[derive(Message)]
pub struct RotaryEncoderMovedMessage(pub RotaryEncoderDirection);