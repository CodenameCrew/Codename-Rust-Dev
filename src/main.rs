use bevy::{
    prelude::*,
    core::FrameCount,
    window::{PresentMode, WindowTheme},
    render::{settings::*, RenderPlugin},
    render::settings::RenderCreation::Automatic
};
use bevy_common_assets::xml::XmlAssetPlugin;
use crate::WgpuSettingsPriority::Functionality;
#[path = "states/state.rs"] mod state;
#[path = "utils/sparrow-loader.rs"] mod sparrow_loader;

// CONSTANTS
// So we can hide the little flash of white while the engine gets ready...
const INIT_FRAMES_BEFORE_SHOW:u32 = 4;
// This is just cause I notice Backends::PRIMARY doesn't choose dx12 on windows, not sure about other platforms
const INIT_BACKEND:Backends = if cfg!(windows) {Backends::DX12} else {Backends::PRIMARY};
// Self explanitory, I think you just use Color::NONE for window transperency after you set it in the WindowPlugin
const INIT_BACKGROUND_COLOR:Color = Color::BLACK;

// INIT
fn main() {
    let mut app = App::new();
    app.add_plugins((
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
            }),
        XmlAssetPlugin::<sparrow_loader::SparrowSheet>::new(&[".xml"]),
    ));
    app.insert_resource(ClearColor(INIT_BACKGROUND_COLOR));
    app.add_systems(Update, make_visible);
    app.add_plugins(state::states_init_plugin);
    app.init_state::<state::GameState>();
    app.init_asset::<sparrow_loader::SparrowSheet>();
    app.run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == INIT_FRAMES_BEFORE_SHOW {
        window.single_mut().visible = true;
    }
}