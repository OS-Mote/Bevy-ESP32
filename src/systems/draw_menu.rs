use bevy::prelude::*;
use esp_println::println;
use crate::{
    components::{
        menu::{
            Menu,
            SelectedMenu
        },
        menu_item::{
            MenuItem,
            SelectedItem
        },
        menu_items::MenuItems
    },
    plugins::main_menu::MainMenuState,
    resources::framebuffer::{
        FrameBuffer,
        FrameBufferResource
    }
};
use embedded_graphics::{
    mono_font::{
        MonoTextStyle,
        ascii::FONT_10X20
    },
    pixelcolor::Rgb565,
    text::{
        Alignment,
        Text
    },
    prelude::*,
};
use embedded_layout::{
    layout::linear::LinearLayout,
    prelude::*
};
use enum_iterator::{
    Sequence,
    next,
    previous
};
use bevy::state::state::FreelyMutableState;

#[derive(PartialEq, Clone, Copy, Sequence)]
enum MenuPosition {
    First,
    Second,
    Third
}

pub fn draw_menu<T: States + Sequence>(
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
    menu_state: Res<State<T>>,
    menu_query: Single<(&Name, &MenuItems), With<SelectedMenu>>,
    menu_items_query: Query<(&MenuItem<T>, &Name, Option<&SelectedItem>)>,
) {

    let (menu_name, menu_items) = menu_query.into_inner();
    let mut menu_item_entities_iterator = menu_items.iter();
    let mut menu_item_entity_chunks_iterator = menu_items.collection().chunks(3);

    while let Some(mut menu_item_entity_chunk) = menu_item_entity_chunks_iterator.next() {
        if menu_item_entity_chunk.iter().any(|x| { menu_items_query.get(*x).unwrap().2.is_some() }) {
            let mut position = MenuPosition::First;
            for menu_item_entity in menu_item_entity_chunk.iter() {
                if let (menu_item, name, selected) = menu_items_query.get(*menu_item_entity).unwrap() {
                    draw_menu_item(
                        name.as_str(),
                        position,
                        selected.is_some(),
                        menu_item.0 == *menu_state.get(),
                        &mut frame_buffer_resource.frame_buffer
                    );

                    position = next(&position).unwrap_or(MenuPosition::First);
                }
            }
        }
    }
}

fn draw_menu_item(name: &str, position: MenuPosition, selected: bool, activated: bool, frame_buffer: &mut FrameBuffer) {
    Text::with_alignment(
        name,
        if position == MenuPosition::First { frame_buffer.bounding_box().center() + Point::new(-100, -10) } else if position == MenuPosition::Second { frame_buffer.bounding_box().center() + Point::new(0, -10) } else { frame_buffer.bounding_box().center() + Point::new(100, -10) },
        MonoTextStyle::new(&FONT_10X20, if activated { Rgb565::GREEN } else if selected { Rgb565::RED } else { Rgb565::BLUE }),
        Alignment::Center,
    )
        .draw(frame_buffer);
}