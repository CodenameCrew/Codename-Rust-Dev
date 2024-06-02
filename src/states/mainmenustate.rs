pub mod main_menu_state {
    use bevy::prelude::*;
    use crate::state::*;

    #[derive(Component)]
    pub struct MainMenuStateMember;

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::MainMenuState), create)
        .add_systems(Update, update.run_if(in_state(GameState::MainMenuState)))
        .add_systems(OnExit(GameState::MainMenuState), destroy)
        .add_systems(OnExit(GameState::MainMenuState), despawn_state::<MainMenuStateMember>);
    }

    pub fn create(mut commands: Commands, _asset_server: Res<AssetServer>) {
        commands.spawn((default_camera(), MainMenuStateMember));
        
        commands.spawn((
            TextBundle::from_section(
                "IT'S ME MAIN MENU STATE!!!\nPress SPACE to switch back...",
                TextStyle{font_size: 20.0, ..default()}
            ).with_style({
                Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    left: Val::Px(14.0),
                    ..default()
                }
            }),
            MainMenuStateMember
        ));
    }

    pub fn update(_time: Res<Time>, keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            game_state.set(GameState::TestingState);
        }
    }

    pub fn destroy(mut _commands: Commands) {}
}