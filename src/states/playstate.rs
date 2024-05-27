pub mod playstate {
    use bevy::prelude::*;
    use crate::state::*;

    #[derive(Component)]
    pub struct PlayStateMember;

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::PlayState), create)
        .add_systems(Update, update.run_if(in_state(GameState::PlayState)))
        .add_systems(OnExit(GameState::PlayState), destroy)
        .add_systems(OnExit(GameState::PlayState), despawn_state::<PlayStateMember>);
    }

    pub fn create(mut commands: Commands, _asset_server: Res<AssetServer>) {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                ..default()
            },
            PlayStateMember
        ));

        commands.spawn((
            TextBundle::from_section(
                "Currently testing chart parsing\nPress SPACE to switch back...",
                TextStyle{font_size: 20.0, ..default()}
            ).with_style({
                Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    left: Val::Px(14.0),
                    ..default()
                }
            }),
            PlayStateMember
        ));
    }

    pub fn update(keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            game_state.set(GameState::TestingState);
        }
    }

    pub fn destroy(mut _commands: Commands) {}
}