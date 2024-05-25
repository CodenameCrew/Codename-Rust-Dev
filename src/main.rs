use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowTheme},
    render::{settings::*, RenderPlugin},
    render::settings::RenderCreation::Automatic, core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings}
};
use crate::WgpuSettingsPriority::Functionality;

// So we can hide the little flash of white while the engine gets ready...
const INIT_FRAMES_BEFORE_SHOW:u32 = 4;
// This is just cause I notice Backends::PRIMARY doesn't choose dx12 on windows, not sure about other platforms
const INIT_BACKEND:Backends = if cfg!(windows) {Backends::DX12} else {Backends::PRIMARY};
// Self explanitory, I think you just use Color::NONE for window transperency after you set it in the WindowPlugin
const INIT_BACKGROUND_COLOR:Color = Color::BLACK;

fn main() {
    App::new()
        // INIT
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                    title: "Friday Night Funkin' - Codename Engine".into(),
                    name: Some("CodenameEngine".into()),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    visible: false,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: Automatic(
                    WgpuSettings {
                        backends: Some(INIT_BACKEND),
                        priority: Functionality,
                        power_preference: PowerPreference::HighPerformance,
                        ..Default::default()
                    }
                ),
                ..default()
            })
        )
        .insert_resource(ClearColor(INIT_BACKGROUND_COLOR))
        // GAME
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                make_visible,
            ),
        )
        .run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == INIT_FRAMES_BEFORE_SHOW {
        window.single_mut().visible = true;
    }
}

// TODO: SET UP A STATES SYSTEM NOW, WE REALLY NEED IT
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        ..default()
    });
}