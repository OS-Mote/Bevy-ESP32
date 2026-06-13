use bevy::{
    prelude::{
        *,
        Transform
    }
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    image::Image
};
use tinytga::Tga;
use crate::FrameBufferResource;

const PUPPIES_TGA_DATA: &[u8; 57742] = include_bytes!("../../resources/puppies.tga");

pub fn draw_puppies(
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
    transforms: Query<&Transform>
) {
    let puppies_tga: Tga<Rgb565> = Tga::from_slice(PUPPIES_TGA_DATA)
        .unwrap();

    for transform in transforms.iter() {
        Image::new(&puppies_tga, Point::new(transform.translation.x as i32, transform.translation.y as i32))
            .draw(&mut frame_buffer_resource.frame_buffer);
    }
}