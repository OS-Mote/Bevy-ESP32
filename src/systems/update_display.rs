use bevy::prelude::*;
use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
};
use crate::DisplayResource;
use crate::FrameBufferResource;

pub fn update_display(
    mut display_resource: ResMut<DisplayResource>,
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
) {
    let fill_area = Rectangle::new(Point::zero(), frame_buffer_resource.frame_buffer.size());

    display_resource
        .display
        .fill_contiguous(&fill_area, frame_buffer_resource.frame_buffer.data.iter().copied())
        .unwrap();
}