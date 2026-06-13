use bevy::prelude::*;
use super::menu_item_of::MenuItemOf;

#[derive(Component)]
#[relationship_target(relationship = MenuItemOf, linked_spawn)]
pub struct MenuItems(Vec<Entity>);