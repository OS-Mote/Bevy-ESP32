use bevy::prelude::*;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
};
use tinytga::Tga;

#[derive(Component)]
pub struct MenuIcon<'a>(pub Tga<'a, Rgb565>);