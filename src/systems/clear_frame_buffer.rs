use bevy::prelude::*;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565
};
use esp_println::println;
use crate::FrameBufferResource;

pub fn clear_frame_buffer(
    mut frame_buffer_resource: ResMut<FrameBufferResource>
) {
    frame_buffer_resource.frame_buffer.clear(Rgb565::BLACK);
}