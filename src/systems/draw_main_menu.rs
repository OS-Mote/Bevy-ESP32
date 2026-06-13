use bevy::prelude::*;
use esp_println::println;
use crate::{
    components::{
        menu::Menu,
        menu_item::MenuItem,
        menu_items::MenuItems
    },
    plugins::main_menu::MainMenuState,
    resources::framebuffer::{FrameBuffer, FrameBufferResource}
};

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    text::{Alignment, Text},
};

pub fn draw_main_menu(
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
    main_menu_state: Res<State<MainMenuState>>,
    menu_query: Single<(&Name, &MenuItems), With<Menu>>,
    menu_items_query: Query<&MenuItem>,
    names_query: Query<&Name>
) {
    let (menu_name, menu_items) = menu_query.into_inner();

    for menu_item_entity in menu_items.iter() {
        let menu_item = menu_items_query.get(menu_item_entity).unwrap();

        if menu_item.0 == **main_menu_state {
            let menu_item_name = names_query.get(menu_item_entity).unwrap().as_str();

            draw_menu_item(menu_item_name, &mut frame_buffer_resource.frame_buffer);
        }
    }
}

fn draw_menu_item(name: &str, frame_buffer: &mut FrameBuffer) {
    Text::with_alignment(
        name,
        frame_buffer.bounding_box().center() + Point::new(0, 15),
        MonoTextStyle::new(&FONT_10X20, Rgb565::BLUE),
        Alignment::Center,
    )
        .draw(frame_buffer);
}