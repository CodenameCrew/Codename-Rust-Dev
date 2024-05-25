use bevy::prelude::*;

// TODO: Find a actual way to do this thats not trash
#[path = "testingstate.rs"] mod testingstate;
#[path = "mainmenustate.rs"] mod mainmenustate;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default] TestingState,
    MainMenuState
}

pub fn states_init_plugin(app: &mut App) {
    app.add_plugins((testingstate::testing_state::plugin, mainmenustate::main_menu_state::plugin));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_state<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}