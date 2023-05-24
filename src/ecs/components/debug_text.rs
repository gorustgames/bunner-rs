use bevy::prelude::*;

#[derive(Component)]
pub struct DebugTextMarker;

#[derive(Bundle)]
pub struct DebugText {
    marker: DebugTextMarker,
    #[bundle]
    text_bundle: TextBundle,
}

impl DebugText {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        DebugText {
            marker: DebugTextMarker,
            text_bundle: TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/ALGER.TTF"),
                                font_size: 15.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/ALGER.TTF"),
                                font_size: 15.0,
                                color: Color::GOLD,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/ALGER.TTF"),
                                font_size: 15.0,
                                color: Color::GREEN,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/ALGER.TTF"),
                                font_size: 15.0,
                                color: Color::BLUE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn spawn_debug_text(self, commands: &mut Commands) {
        commands.spawn_bundle(self);
    }
}
