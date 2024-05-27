use bevy::{
    prelude::*,
    core::FrameCount,
    window::{PresentMode, WindowTheme, WindowResolution},
    render::{settings::*, RenderPlugin},
    render::settings::RenderCreation::Automatic
};
use crate::WgpuSettingsPriority;

#[path = "states/state.rs"] mod state;
#[path = "util/sparrow.rs"] mod sparrow;
#[path = "util/fpscounter.rs"] mod fpscounter;
#[path = "data/chart.rs"] mod chart;
use chart::*;

// So we can hide the little flash of white while the engine gets ready...
const INIT_FRAMES_BEFORE_SHOW:u32 = 4;
// Self explanitory, I think you just use Color::NONE for window transperency after you set it in the WindowPlugin
const INIT_BACKGROUND_COLOR:Color = Color::BLACK;

fn main() {
    let test: &str = "{
        \"strumScale\": 1,
        \"position\": \"girlfriend\",
        \"notes\": [],
        \"characters\": [
          \"gf\"
        ],
        \"type\": 2,
        \"scrollSpeed\": null,
        \"visible\": false,
        \"strumPos\": [
          0,
          50
        ],
        \"strumLinePos\": 0.5
    }";
    println!("{}", test);

    let parsed: ChartStrumLine = serde_json::from_str(test).unwrap();
    match parsed.r#type {
        ChartStrumLineType::ADDITIONAL => println!("ADDITIONAL"),
        ChartStrumLineType::OPPONENT => println!("OPPONENT"),
        ChartStrumLineType::PLAYER => println!("PLAYER"),
        _ => panic!("Unknown strumline type: {}", parsed.r#type),
    }

    let mut app = App::new();
    app.add_plugins((
        // INTERNAL STUFF
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Friday Night Funkin' - Codename Engine".into(),
                    name: Some("CodenameEngine".into()),
                    resolution:  WindowResolution::new(1280., 720.).with_scale_factor_override(1.0),
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
                        backends: Some(Backends::PRIMARY),
                        priority: WgpuSettingsPriority::Functionality,
                        power_preference: PowerPreference::HighPerformance,
                        ..Default::default()
                    }
                ),
                ..default()
            }),
    ));
    app.insert_resource(ClearColor(INIT_BACKGROUND_COLOR));
    app.add_systems(Update, make_visible);
    // STATES
    app.add_plugins(state::states_init_plugin);
    app.init_state::<state::GameState>();
    // EXTRA STUFF
    app.add_systems(Update, sparrow::update);
    app.add_plugins(fpscounter::fps_plugin);
    app.run();
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == INIT_FRAMES_BEFORE_SHOW {
        window.single_mut().visible = true;
    }
}