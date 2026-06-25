use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::ecs::system::IntoResult;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use esp_println::println;
use crate::components::menu::Menu;
use crate::components::menu_item::SelectedItem;
use crate::components::menu_items::MenuItems;
use crate::events::rotary_encoder_button_pressed_event::RotaryEncoderButtonPressedEvent;
use crate::{components::menu_item::MenuItem, plugins::main_menu::MainMenuState};
use crate::rotary_encoder::RotaryEncoderDirection;
use enum_iterator::{
    Sequence, all, next, previous
};

pub fn process_menu_input (
    // mut commands: Commands,
    // mut rotary_encoder_moved_message_reader: MessageReader<RotaryEncoderMovedMessage>,
    // mut rotary_encoder_button_pressed_message_reader: MessageReader<RotaryEncoderButtonPressedMessage>,
    // mut next_menu_state: ResMut<NextState<T>>,
    // menu_entity_query: Single<Entity, With<M>>,
    // menu_query: Single<(&Name, &MenuItems), With<M>>,
    // menu_items_query: Query<Option<&SelectedItem>, With<MenuItem<T>>>,
    // selected_menu_item_query: Single<&MenuItem<T>, With<SelectedItem>>
) {
    // let (menu_name, menu_items) = menu_query.into_inner();

    // for message in rotary_encoder_moved_message_reader.read() {
    //     match message.0 {
    //         RotaryEncoderDirection::Clockwise => {
    //             let mut menu_item_entities_iterator = menu_items.iter().peekable();
                
    //             while let (Some(menu_item_entity), Some(next_menu_item_entity)) = (menu_item_entities_iterator.next(), menu_item_entities_iterator.peek()) {
    //                 if menu_items_query.get(menu_item_entity).unwrap().is_some() {
    //                     commands.entity(menu_item_entity).remove::<SelectedItem>();
    //                     commands.entity(*next_menu_item_entity).insert(SelectedItem);
    //                 }
    //             }
    //         },
    //         RotaryEncoderDirection::Anticlockwise => {
    //             let mut menu_item_entities_iterator = menu_items.iter().peekable();
                
    //             while let (Some(previous_menu_item_entity), Some(menu_item_entity)) = (menu_item_entities_iterator.next(), menu_item_entities_iterator.peek()) {
    //                 if menu_items_query.get(*menu_item_entity).unwrap().is_some() {
    //                     commands.entity(*menu_item_entity).remove::<SelectedItem>();
    //                     commands.entity(previous_menu_item_entity).insert(SelectedItem);
    //                 }
    //             }
    //         }
    //     }
    // }

    // let selected_menu_item = selected_menu_item_query.into_inner();

    // for _ in rotary_encoder_button_pressed_message_reader.read() {
    //     next_menu_state.set(selected_menu_item.0);
    // }
}