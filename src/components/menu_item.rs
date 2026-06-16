use bevy::prelude::*;
use crate::plugins::main_menu::MainMenuState;
use enum_iterator::Sequence;

#[derive(Component)]
pub struct SelectedItem;

#[derive(Component)]
pub struct MenuItem<T: States>(pub T);