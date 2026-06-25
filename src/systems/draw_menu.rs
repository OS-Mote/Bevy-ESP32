use bevy::{ecs::entity::unique_slice::ChunkBy, prelude::*};
use esp_println::println;
use crate::{
    components::{
        menu::Menu, menu_icon::MenuIcon, menu_item::{
            MenuItem,
            SelectedItem
        }, menu_items::MenuItems
    }, plugins::main_menu::MainMenuState, resources::{display::{self, Display, DisplayResource}, framebuffer::{
        FrameBuffer,
        FrameBufferResource
    }}, sprite_buffer::SpriteBuffer
};
use embedded_graphics::{
    image::Image, mono_font::{
        MonoTextStyle,
        ascii::FONT_10X20
    }, pixelcolor::Rgb565, prelude::*, primitives::Rectangle, text::{
        Alignment,
        Text
    },
    primitives::PrimitiveStyle
};
use embedded_layout::{
    layout::{self, linear::{LinearLayout, Vertical, spacing::DistributeFill}}, prelude::*, view_group::ViewGroup
};
use embedded_layout::layout::linear::FixedMargin;
use enum_iterator::{
    Sequence,
    next,
    previous
};
use bevy::state::state::FreelyMutableState;
use tinytga::Tga;

#[derive(PartialEq, Clone, Copy, Sequence)]
enum MenuPosition {
    First,
    Second,
    Third
}

pub fn draw_menu<M: Component, T: States + Sequence>(
    mut frame_buffer_resource: ResMut<FrameBufferResource>,
    display_resource: Res<DisplayResource>,
    menu_state: Res<State<T>>,
    menu_query: Single<(&Name, &MenuItems), With<M>>,
    menu_items_query: Query<(&MenuItem<T>, &Name, Option<&MenuIcon<'static>>, Option<&SelectedItem>)>,
) {
    let (menu_name, menu_items) = menu_query.into_inner();
    let empty_tga: Tga<Rgb565> = Tga::from_slice(include_bytes!("../../resources/empty.tga")).unwrap();
    
    let menu_items = menu_items
        .collection()
        .iter()
        .flat_map(|entity|
            menu_items_query.get(*entity).ok()
        )
        .collect::<Vec<_>>();
    
    let selected_menu_items_chunk = menu_items
        .chunks(3)
        .find(|chunk|
            chunk
                .iter()
                .any(|(_, _, _, selected)|
                    selected.is_some()
                )
        )
        .unwrap();

    let mut selected_menu_items_chunk_layouts = selected_menu_items_chunk
        .iter()
        .map(|(menu_item, name, menu_icon, selected)|
            LinearLayout::vertical(
                Chain::new(
                    Image::new(
                            if let Some(menu_icon) = menu_icon { &menu_icon.0 } else { &empty_tga },
                            Point::zero()
                        )
                )
                .append(
                    Text::with_alignment(
                        name,
                    Point::zero(),
                    MonoTextStyle::new(&FONT_10X20, if menu_item.0 == *menu_state.get() { Rgb565::GREEN } else if selected.is_some() { Rgb565::RED } else { Rgb565::BLUE }),
                            Alignment::Center
                    )
                )
            )
                .with_alignment(horizontal::Center)
                .arrange()
        )
        .collect::<Vec<_>>();

    LinearLayout::horizontal(Views::new(selected_menu_items_chunk_layouts.as_mut_slice()))
        .with_spacing(FixedMargin(50))
        .with_alignment(vertical::Center)
        .arrange()
        .align_to(&display_resource.display.bounding_box(), horizontal::Center, vertical::Center)
        .draw(&mut SpriteBuffer{ frame_buffer: &mut frame_buffer_resource.frame_buffer })
        .unwrap();

    // while let Some(menu_item_entity_chunk) = menu_item_entity_chunks_iterator.next() {
    //     if menu_item_entity_chunk.iter().any(|x| { menu_items_query.get(*x).unwrap().3.is_some() }) {
    //         // let mut f = menu_item_entity_chunk.iter().flat_map(|x| menu_items_query.get(*x)).map(|(menu_item, name, menu_icon, selected)|
    //             LinearLayout::vertical(
    //                 Chain::new(
    //                     Image::new(
    //                             if let Some(menu_icon) = menu_icon { &menu_icon.0 } else { &empty_tga },
    //                             Point::zero()
    //                         )
    //                 )
    //                 .append(
    //                     Text::with_alignment(
    //                         name,
    //                     Point::zero(),
    //                     MonoTextStyle::new(&FONT_10X20, if menu_item.0 == *menu_state.get() { Rgb565::GREEN } else if selected.is_some() { Rgb565::RED } else { Rgb565::BLUE }),
    //                             Alignment::Center
    //                     )
    //                 )
    //             )
    //                 .arrange()
            // );

            // let mut chain = Chain::new(f.next().next().unwrap());

            // f.for_each(|layout|{
                
            // });

            // for layout in f {
            //     chain.clone().append(layout);
            // }
            // let chain = Chain::new(layouts);
            // let (menu_item, name, menu_icon, selected) = menu_items_chunk_iterator.next().unwrap();



            // let chain = Chain::new(
            //     LinearLayout::vertical(
            //         Chain::new(
            //             Image::new(
            //                     if let Some(menu_icon) = menu_icon { &menu_icon.0 } else { &empty_tga },
            //                     Point::zero()
            //                 )
            //         )
            //         .append(
            //             Text::with_alignment(
            //                 name,
            //             Point::zero(),
            //             MonoTextStyle::new(&FONT_10X20, if menu_item.0 == *menu_state.get() { Rgb565::GREEN } else if selected.is_some() { Rgb565::RED } else { Rgb565::BLUE }),
            //                     Alignment::Center
            //             )
            //         )
            //     )
            //     .arrange()
            // )
            //     .append(
            //         Rectangle::new(Point::zero(), Size::new(16, 16))
            //         .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
            //     );

            // while let (menu_item, name, menu_icon, selected) = menu_items_chunk_iterator.next().unwrap() {
            //     chain.clone().append(
            //         LinearLayout::vertical(
            //             Chain::new(
            //                 Image::new(
            //                         if let Some(menu_icon) = menu_icon { &menu_icon.0 } else { &empty_tga },
            //                         Point::zero()
            //                     )
            //             )
            //             .append(
            //                 Text::with_alignment(
            //                 name,
            //             Point::zero(),
            //             MonoTextStyle::new(&FONT_10X20, if menu_item.0 == *menu_state.get() { Rgb565::GREEN } else if selected.is_some() { Rgb565::RED } else { Rgb565::BLUE }),
            //                     Alignment::Center
            //                 )
            //             )
            //         )
            //         .arrange()
            //     );
            // }
            
            // LinearLayout::horizontal(chain)
            //     .with_alignment(vertical::Center)
            //     .arrange()
            //     .align_to(&display_resource.display.bounding_box(), horizontal::Center, vertical::Center)
            //     .draw(&mut frame_buffer_resource.frame_buffer)
            //     .unwrap();
        // }
    // }
}

fn draw_menu_item(name: &str, icon: Option<Tga<Rgb565>>, position: MenuPosition, selected: bool, activated: bool, frame_buffer: &mut FrameBuffer, display: &Display) {
    let icon_image = if let Some(icon) = icon { icon } else { Tga::from_slice(include_bytes!("../../resources/empty.tga")).unwrap() };
    
    LinearLayout::vertical(
        Chain::new(
            Image::new(
                    &icon_image,
                    Point::zero()
                )
        )
        .append(
            Text::with_alignment(
            name,
        Point::zero(),
        MonoTextStyle::new(&FONT_10X20, if activated { Rgb565::GREEN } else if selected { Rgb565::RED } else { Rgb565::BLUE }),
                Alignment::Center
            )
        )
        // .append(
        //     Rectangle::zero()
        //     .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        // )
    )
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&display.bounding_box(), horizontal::Center, vertical::Center)
    .draw(frame_buffer)
    .unwrap();

    
    
    // Text::with_alignment(
    //     name,
    //     if position == MenuPosition::First { frame_buffer.bounding_box().center() + Point::new(-100, 20) } else if position == MenuPosition::Second { frame_buffer.bounding_box().center() + Point::new(0, 20) } else { frame_buffer.bounding_box().center() + Point::new(100, 20) },
    //     MonoTextStyle::new(&FONT_10X20, if activated { Rgb565::GREEN } else if selected { Rgb565::RED } else { Rgb565::BLUE }),
    //     Alignment::Center,
    // )
    //     .draw(frame_buffer);
}