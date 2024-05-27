use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*
};

#[derive(Component)]
pub struct FPSCounter;

pub fn fps_plugin(app: &mut App) {
    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ));

    app.add_systems(Startup, fps_setup);
    app.add_systems(Update, fps_update);
}

pub fn fps_setup(mut commands: Commands) {
    commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(14.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            z_index: ZIndex::Global(i32::MAX),
            background_color: Color::BLACK.with_a(0.75).into(),
            ..default()
    }).with_children(|builder| {
        builder.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle{font_size: 16.0, color: Color::WHITE, ..default()}
                ),
                TextSection::new(
                    "120",
                    TextStyle{font_size: 16.0, color: Color::GREEN, ..default()}
                ),
                TextSection::new(
                    " (",
                    TextStyle{font_size: 16.0, color: Color::WHITE, ..default()}
                ),
                TextSection::new(
                    "80",
                    TextStyle{font_size: 16.0, color: Color::ORANGE, ..default()}
                ),
                TextSection::new(
                    ")",
                    TextStyle{font_size: 16.0, color: Color::WHITE, ..default()}
                )
            ]),
            FPSCounter
        ));
    });
}

pub fn fps_update(
    diagnostics: Res<DiagnosticsStore>, 
    mut fps_counter: Query<&mut Text, With<FPSCounter>>
) {
    let mut fps_text = fps_counter.single_mut();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(smooth) = fps.smoothed() {
            fps_text.sections[1].value = format!("{smooth:.2}");
        }
    }

    if let Some(frametime) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(smooth) = frametime.smoothed() {
            fps_text.sections[3].value = format!("{smooth:.4}");
        }
    }
}