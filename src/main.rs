
use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowTheme},
    render::{settings::*, RenderPlugin},
    render::settings::RenderCreation::Automatic
};
use crate::WgpuSettingsPriority::Functionality;

const INIT_FRAMES_BEFORE_SHOW:u32 = 4;
const INIT_BACKEND:Backends = if cfg!(windows) {Backends::DX12} else {Backends::PRIMARY} ;
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

                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after INIT_FRAMES_BEFORE_SHOW frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
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
        // Game stuff
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
    // The delay may be different for your app or system.
    if frames.0 == INIT_FRAMES_BEFORE_SHOW {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

// TODO: SET UP A STATES SYSTEM NOW, WE REALLY NEED IT
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        ..default()
    });
}