use bevy::prelude::*;

use crate::{components::settings_menu::SettingsMenu, plugins::main_menu::MainMenuState, resources::navigation_stack::NavigationStack};

pub fn cleanup_settings_menu(
    mut commands: Commands,
    menu_state: Res<State<MainMenuState>>,
    mut navigation_stack: ResMut<NavigationStack>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    settings_menu_entity_query: Single<Entity, With<SettingsMenu>>
) {
    next_menu_state.set(MainMenuState::None);

    let settings_menu_entity = settings_menu_entity_query.into_inner();

    if let Some(entity) = navigation_stack.last() {
        if *entity == settings_menu_entity {
            navigation_stack.pop();
        }
    }

    commands.entity(settings_menu_entity).despawn();
}