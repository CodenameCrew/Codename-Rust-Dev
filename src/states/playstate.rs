pub mod playstate {
    use bevy::prelude::*;
    use bevy_kira_audio::prelude::*;
    use crate::{
        state::*, 
        chart::{ChartData, self}, 
        paths,
    };

    #[derive(Component)]
    pub struct PlayStateMember;

    #[derive(Resource)]
    pub struct PlayStateTracks;


    #[derive(Component)]
    pub struct ChartComponent {
        pub chart: ChartData
    }

    #[derive(Component)]
    pub struct AudioHandle(Handle<AudioInstance>);

    #[derive(Component)]
    pub struct MainTrackComponent;

    pub fn plugin(app: &mut App) {
        app
        .add_systems(OnEnter(GameState::PlayState), create)
        .add_systems(Update, update.run_if(in_state(GameState::PlayState)))
        .add_systems(Update, audio_update.run_if(in_state(GameState::PlayState)))
        .add_systems(OnExit(GameState::PlayState), destroy)
        .add_systems(OnExit(GameState::PlayState), despawn_state::<PlayStateMember>)
        .add_audio_channel::<PlayStateTracks>();
    }

    pub fn create(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<AudioChannel<PlayStateTracks>>) {
        commands.spawn((default_camera(), PlayStateMember));

        let chart_component: ChartComponent = load_chart(&"smoke and mirrors".to_owned(), Some(&"hard".to_owned()));
        commands.spawn((chart_component, PlayStateMember));

        let inst_file = asset_server.load(paths::inst(&"smoke and mirrors".to_owned(), Some(&"hard".to_owned())));
        let voices_file = asset_server.load(paths::voices(&"smoke and mirrors".to_owned(), Some(&"hard".to_owned()), None));

        let inst_handle: AudioHandle = AudioHandle(audio.play(inst_file).with_volume(0.3).handle());
        let voices_handle: AudioHandle = AudioHandle(audio.play(voices_file).with_volume(0.3).handle());

        commands.spawn((inst_handle, MainTrackComponent, PlayStateMember));
        commands.spawn((voices_handle, PlayStateMember));

        // DEBUG TEXT
        commands.spawn((
            TextBundle::from_section(
                "Currently testing strumlines...\nPress SPACE to switch back...",
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

    pub fn load_chart(song: &String, difficulty: Option<&String>) -> ChartComponent {
        return match chart::parser::parse(song, difficulty) {
            Some(chart) => ChartComponent {chart: chart},
            None => {
                println!("CHART {}.json NOT FOUND!", paths::chart(song, difficulty));
                ChartComponent {chart: chart::BASE_CHART}
            }
        };
    }

    pub fn update(
        keyboard_input: Res<ButtonInput<KeyCode>>, 
        mut game_state: ResMut<NextState<GameState>>
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            game_state.set(GameState::TestingState);
        }
    }

    pub fn audio_update(
        mut audio_instances: ResMut<Assets<AudioInstance>>,
        mut main_audio_query:Query<&AudioHandle, With<MainTrackComponent>>
    ) {
        let main_audio_handle = main_audio_query.single_mut();

        if let Some(audio) = audio_instances.get_mut(&main_audio_handle.0) {
            match audio.state() {
                PlaybackState::Playing { position } => {
                    println!("{}", position);
                },
                _ => todo!()
            }
        }
    }

    pub fn destroy(mut _commands: Commands, audio: Res<AudioChannel<PlayStateTracks>>) {
        audio.stop();
    }
}