// Alot of this derived from https://github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs with a few changes
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

// ! THIS IS A TEMPLATE FOR A GENERIC STATE, DO NOT DELETE!!!!
// ! ADD TO THE ENUM GameState AND MAKE A COMPONENT FOR MEMBERS OF YOUR STATE
/*
pub mod state_name_state {
    use bevy::prelude::*;
    use crate::state::*;

    #[derive(Component)]
    pub struct StateNameMember;

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::StateNameState), create)
        .add_systems(Update, update.run_if(in_state(GameState::StateNameState)))
        .add_systems(OnExit(GameState::StateNameState), destroy)
        .add_systems(OnExit(GameState::StateNameState), despawn_state::<StateNameMember>);
    }

    pub fn create(mut commands: Commands, _asset_server: Res<AssetServer>) {
        // ! WHEN YOU ADD USE THE STATE MEMBER COMPEMENT StateNameMember
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                ..default()
            },
            StateNameMember
        ));
    }

    pub fn update(_time: Res<Time>, keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
        let mut elapsed:f32 = time.delta_seconds();
    }

    pub fn destroy(mut _commands: Commands) {}
}
*/