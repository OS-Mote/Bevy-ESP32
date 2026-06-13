use bevy::{
    prelude::{
        *,
        Transform
    }
};
use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
    pixelcolor::Rgb565,
    image::Image
};
use tinytga::Tga;
use crate::DisplayResource;
use crate::FrameBufferResource;

const PUPPIES_TGA_DATA: &[u8; 57742] = include_bytes!("../../resources/puppies.tga");

pub fn render(
    mut display_resource: ResMut<DisplayResource>,
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
    transforms: Query<&Transform>
) {
    frame_buffer_resource.frame_buffer.clear(Rgb565::BLACK);

    let puppies_tga: Tga<Rgb565> = Tga::from_slice(PUPPIES_TGA_DATA)
        .unwrap();

    for transform in transforms.iter() {
        Image::new(&puppies_tga, Point::new(transform.translation.x as i32, transform.translation.y as i32))
            .draw(&mut frame_buffer_resource.frame_buffer);
    }

    let fill_area = Rectangle::new(Point::zero(), frame_buffer_resource.frame_buffer.size());

    display_resource.display
        .fill_contiguous(&fill_area, frame_buffer_resource.frame_buffer.data.iter().copied())
        .unwrap();
}