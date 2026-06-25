use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{
    geometry::{OriginDimensions, Size},
    prelude::{DrawTarget, Pixel},
};
use embedded_graphics_framebuf::FrameBuf;
use embedded_graphics_framebuf::backends::FrameBufferBackend;

pub struct SpriteBuffer<'a, B: FrameBufferBackend<Color = Rgb565>> {
    pub frame_buffer: &'a mut FrameBuf<Rgb565, B>,
}

impl<'a, B> OriginDimensions for SpriteBuffer<'a, B>
where
    B: embedded_graphics_framebuf::backends::FrameBufferBackend<Color = Rgb565>,
{
    fn size(&self) -> Size {
        self.frame_buffer.size()
    }
}

impl<'a, B> DrawTarget for SpriteBuffer<'a, B>
where
    B: FrameBufferBackend<Color = Rgb565>,
{
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Filter out "magic pink" (RGB565: R==31, G==0, B==31)
            if color.r() == 31 && color.g() == 0 && color.b() == 31 {
                continue;
            }
            if coord.x >= 0
                && coord.x < self.frame_buffer.width() as i32
                && coord.y >= 0
                && coord.y < self.frame_buffer.height() as i32
            {
                self.frame_buffer.set_color_at(coord, color);
            }
        }
        Ok(())
    }
}

impl<'a, B> SpriteBuffer<'a, B>
where
    B: FrameBufferBackend<Color = Rgb565>,
{
    pub fn get_pixel_iter(&self) -> impl Iterator<Item = Rgb565> + '_ {
        self.frame_buffer.into_iter().map(|pixel| pixel.1)
    }
}