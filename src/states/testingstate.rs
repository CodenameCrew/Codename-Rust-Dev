use bevy::prelude::*;

#[derive(Component)]
pub struct TestingStateMember;
#[derive(Component)]
pub struct DumbassBird;

pub mod testing_state {
    use bevy::{
        prelude::*,
        core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings}
    };
    use crate::state::*;
    use super::TestingStateMember;
    use super::DumbassBird;

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::TestingState), create)
        .add_systems(Update, update.run_if(in_state(GameState::TestingState)))
        .add_systems(OnExit(GameState::TestingState), destroy)
        .add_systems(OnExit(GameState::TestingState), despawn_state::<TestingStateMember>);
    }

    pub fn create(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            BloomSettings::default(),
            TestingStateMember
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(0.4, 0.4, 0.4),
                    ..default()
                },
                texture: asset_server.load("bevy_bird_dark.png"),
                ..default()
            },
            DumbassBird,
            TestingStateMember
        ));
        commands.spawn((
            TextBundle::from_section(
                "Press SPACE to switch to another state",
                TextStyle{font_size: 32.0, ..default()}
            ),
            TestingStateMember
        ));
    
    }

    pub fn update(time: Res<Time>, mut dumbass_bird_query:Query<&mut Transform, With<DumbassBird>>, keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
        let mut bird_transform = dumbass_bird_query.single_mut();
        let time_passed:f32 = time.elapsed().as_secs_f32() * 2.0;

        bird_transform.translation.y = time_passed.cos()*30.0;

        if keyboard_input.just_pressed(KeyCode::Space) {
            game_state.set(GameState::MainMenuState);
        }
    }

    pub fn destroy(mut _commands: Commands) {}
}