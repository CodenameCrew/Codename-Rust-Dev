pub mod testing_state {
    use bevy::{
        prelude::*,
        core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings},
        render::camera::ScalingMode
    };
    use crate::state::*;
    use crate::sparrow::SpriteXMLBundle;

    #[derive(Component)]
    pub struct TestingStateMember;
    #[derive(Component)]
    pub struct SwingyFella {
        pub x_amount:f32,
        pub y_amount:f32,
    
        pub x_speed:f32,
        pub y_speed:f32,
    }

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::TestingState), create)
        .add_systems(Update, update.run_if(in_state(GameState::TestingState)))
        .add_systems(OnExit(GameState::TestingState), destroy)
        .add_systems(OnExit(GameState::TestingState), despawn_state::<TestingStateMember>);
    }

    pub fn create(
        mut commands: Commands, asset_server: Res<AssetServer>, 
        texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
    ) {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                /*
                projection: OrthographicProjection {
                    // don't forget to set `near` and `far`
                    near: -1000.0,
                    far: 1000.0,
                    scaling_mode: ScalingMode::FixedVertical(720.0),
                    // ... any other settings you want to change ...
                    ..default()
                },
                */
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            BloomSettings::default(),
            TestingStateMember
        ));

        match SpriteXMLBundle::new("images/logoBumpin".to_string(), &asset_server, texture_atlas_layouts) {
            Some(mut spritesheet) => {
                let mut spriteatlas: &mut TextureAtlas = &mut spritesheet.sprite_sheet.atlas;
                spritesheet.spritexml.add_anim_from_prefix("logo bumpin".to_string(), true, 24);
                spritesheet.spritexml.apply_offsets(&mut spriteatlas, &mut spritesheet.sprite_sheet.transform);

                spritesheet.spritexml.set_anim("".to_string(), &mut spriteatlas, true);

                spritesheet.sprite_sheet.transform.scale = Vec3::new(0.5, 0.5, 0.5);

                commands.spawn((
                    spritesheet,
                    TestingStateMember,
                    SwingyFella {
                        x_amount: 60.0,
                        y_amount: 90.0,

                        x_speed: 2.0,
                        y_speed: 1.0
                    }
                ));
            }
            None => {println!("Error loading XML!");}
        }

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(0.4, 0.4, 0.4),
                    translation: Vec3::new(1280.0/2.0 - 200.0, 720.0/2.0 - 100.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.5, 1.5, 1.5), // 4. Put something bright in a dark environment to see the effect
                    ..default()
                },
                texture: asset_server.load("images/bevy_bird_dark.png"),
                ..default()
            },
            TestingStateMember
        ));

        commands.spawn((
            TextBundle::from_section(
                "Press SPACE to switch to another state...\nPress P to switch to PlayState...",
                TextStyle{font_size: 20.0, ..default()}
            ).with_style(
                Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    left: Val::Px(14.0),
                    ..default()
                }
            ),
            TestingStateMember
        ));
    }

    pub fn update(
        time: Res<Time>, 
        mut swingy_fellas:Query<(&SwingyFella, &mut Transform)>, keyboard_input: Res<ButtonInput<KeyCode>>, 
        mut game_state: ResMut<NextState<GameState>>
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            game_state.set(GameState::MainMenuState);
            return;
        }

        if keyboard_input.just_pressed(KeyCode::KeyP) {
            game_state.set(GameState::PlayState);
            return;
        }

        let time_passed:f32 = time.elapsed().as_secs_f32() * 2.0;
        for (&ref swingy_fella, mut transform) in swingy_fellas.iter_mut() {
            transform.translation.x = (time_passed * swingy_fella.x_speed).sin() * swingy_fella.x_amount;
            transform.translation.y = (time_passed * swingy_fella.y_speed).cos() * swingy_fella.y_amount;
        }
    }

    pub fn destroy(mut _commands: Commands) {}
}