use bevy::prelude::*;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565
};
use embedded_graphics_framebuf::FrameBuf;

const FRAME_BUFFER_HORIZONTAL_SIZE: usize = 320;
const FRAME_BUFFER_VERTICAL_SIZE: usize = 240;
const FRAME_BUFFER_SIZE: usize = FRAME_BUFFER_HORIZONTAL_SIZE * FRAME_BUFFER_VERTICAL_SIZE;

pub type FrameBuffer = FrameBuf<Rgb565, [Rgb565; FRAME_BUFFER_SIZE]>;

#[derive(Resource)]
pub struct FrameBufferResource {
    pub frame_buffer: FrameBuffer
}

impl FrameBufferResource {
    pub fn new() -> Self {
        let frame_buffer = FrameBuffer::new([Rgb565::BLACK; FRAME_BUFFER_SIZE], FRAME_BUFFER_HORIZONTAL_SIZE, FRAME_BUFFER_VERTICAL_SIZE);
        
        Self { frame_buffer }
    }
}