use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::{components::{menu_item::{MenuItem, SelectedItem}, menu_items::MenuItems}, events::rotary_encoder_rotated_event::RotaryEncoderRotatedEvent, rotary_encoder::RotaryEncoderDirection};
use enum_iterator::Sequence;

pub fn on_rotary_encoder_rotated<M: Component, T: States + FreelyMutableState + Sequence + Copy>(
    event: On<RotaryEncoderRotatedEvent>,
    mut commands: Commands,
    menu_query: Single<(&Name, &MenuItems), With<M>>,
    menu_items_query: Query<Option<&SelectedItem>, With<MenuItem<T>>>,
    selected_menu_item_query: Single<&MenuItem<T>, With<SelectedItem>>,
) {
    let (menu_name, menu_items) = menu_query.into_inner();
    let mut menu_item_entities_iterator = menu_items.iter().peekable();

    match event.direction {
        RotaryEncoderDirection::Clockwise => {
            while let (Some(menu_item_entity), Some(next_menu_item_entity)) = (menu_item_entities_iterator.next(), menu_item_entities_iterator.peek()) {
                if menu_items_query.get(menu_item_entity).unwrap().is_some() {
                    commands.entity(menu_item_entity).remove::<SelectedItem>();
                    commands.entity(*next_menu_item_entity).insert(SelectedItem);
                }
            }
        }
        RotaryEncoderDirection::Anticlockwise => {
            while let (Some(previous_menu_item_entity), Some(menu_item_entity)) = (menu_item_entities_iterator.next(), menu_item_entities_iterator.peek()) {
                if menu_items_query.get(*menu_item_entity).unwrap().is_some() {
                    commands.entity(*menu_item_entity).remove::<SelectedItem>();
                    commands.entity(previous_menu_item_entity).insert(SelectedItem);
                }
            }
        }
    }
}