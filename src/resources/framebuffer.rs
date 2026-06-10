use bevy::prelude::*;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565
};
use embedded_graphics_framebuf::FrameBuf;

use alloc::boxed::Box;
use embedded_graphics_framebuf::backends::FrameBufferBackend;

pub struct HeapBuffer<C: PixelColor, const N: usize>(Box<[C; N]>);

impl<C: PixelColor, const N: usize> HeapBuffer<C, N> {
    pub fn new(data: Box<[C; N]>) -> Self {
        Self(data)
    }
}

impl<C: PixelColor, const N: usize> core::ops::Deref for HeapBuffer<C, N> {
    type Target = [C; N];
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<C: PixelColor, const N: usize> core::ops::DerefMut for HeapBuffer<C, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<C: PixelColor, const N: usize> FrameBufferBackend for HeapBuffer<C, N> {
    type Color = C;
    fn set(&mut self, index: usize, color: Self::Color) {
        self.0[index] = color;
    }
    fn get(&self, index: usize) -> Self::Color {
        self.0[index]
    }
    fn nr_elements(&self) -> usize {
        N
    }
}

const FRAME_BUFFER_HORIZONTAL_SIZE: usize = 320;
const FRAME_BUFFER_VERTICAL_SIZE: usize = 240;
const FRAME_BUFFER_SIZE: usize = FRAME_BUFFER_HORIZONTAL_SIZE * FRAME_BUFFER_VERTICAL_SIZE;

pub type FrameBuffer = FrameBuf<Rgb565, HeapBuffer<Rgb565, FRAME_BUFFER_SIZE>>;

#[derive(Resource)]
pub struct FrameBufferResource {
    pub frame_buffer: FrameBuffer
}

impl FrameBufferResource {
    pub fn new() -> Self {
        let frame_buffer_data = Box::new([Rgb565::BLACK; FRAME_BUFFER_SIZE]);
        let heap_buffer = HeapBuffer::new(frame_buffer_data);
        let frame_buffer = FrameBuffer::new(heap_buffer, FRAME_BUFFER_HORIZONTAL_SIZE, FRAME_BUFFER_VERTICAL_SIZE);
        
        Self { frame_buffer }
    }
}