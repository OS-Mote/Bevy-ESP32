use bevy::prelude::*;
use super::menu_items::MenuItems;

#[derive(Component)]
#[relationship(relationship_target = MenuItems)]
pub struct MenuItemOf(Entity);