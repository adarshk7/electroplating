use bevy::audio::PlaybackMode;
use bevy::prelude::*;

use crate::asset_loader::AudioAssets;

#[derive(Debug, Component)]
pub struct Plate {
    id: usize,
    pub state: PlateState,
    selected: bool,
}

impl Plate {
    pub fn new(id: usize) -> Self {
        Plate {
            id,
            state: PlateState::Off,
            selected: false,
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            PlateState::Positive => PlateState::Negative,
            PlateState::Negative => PlateState::Positive,
            PlateState::Off => PlateState::Positive,
        }
    }

    pub fn off(&mut self) {
        self.state = PlateState::Off
    }
}

#[derive(Resource)]
pub struct PolarityIndicatorBoard {
    pub polarity: PlateState,
}

#[derive(Copy, Clone, Debug)]
pub enum PlateState {
    Positive,
    Negative,
    Off,
}

pub fn polarity_indicator_board_system(
    polarity_indicator_board: Res<PolarityIndicatorBoard>,
    mut query: Query<&mut Text>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = match polarity_indicator_board.polarity {
            PlateState::Off => "OFF".to_string(),
            PlateState::Positive => "+".to_string(),
            PlateState::Negative => "-".to_string(),
        };
    }
}

#[derive(Debug, Resource)]
pub struct PlateSelectedAnimationTimer {
    pub timer: Timer,
}

pub fn plate_control_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Plate, &mut Visibility)>,
    mut animation_timer: ResMut<PlateSelectedAnimationTimer>,
    audio_assets: Res<AudioAssets>,
    mut polarity_indicator_board: ResMut<PolarityIndicatorBoard>,
) {
    let mut plate_count = 1;
    let mut plate_selected = 1;
    for (mut plate, _) in query.iter_mut() {
        if plate.selected {
            if keyboard_input.just_pressed(KeyCode::Up) {
                commands.spawn(AudioSourceBundle {
                    source: audio_assets.blip5.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
                plate.toggle();
            }
            if keyboard_input.just_pressed(KeyCode::Down) {
                commands.spawn(AudioSourceBundle {
                    source: audio_assets.blip3.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                });
                plate.off();
            }
            plate_selected = plate.id;
        }
        plate_count = plate_count.max(plate.id);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        commands.spawn(AudioSourceBundle {
            source: audio_assets.blip8.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
        plate_selected -= 1;
        if plate_selected < 1 {
            plate_selected = plate_count;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        commands.spawn(AudioSourceBundle {
            source: audio_assets.blip8.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
        plate_selected += 1;
        if plate_selected > plate_count {
            plate_selected = 1;
        }
    }
    for (mut plate, mut visible) in query.iter_mut() {
        plate.selected = plate_selected == plate.id;
        if plate.selected {
            if animation_timer.timer.tick(time.delta()).just_finished() {
                if *visible == Visibility::Visible {
                    *visible = Visibility::Hidden;
                } else {
                    *visible = Visibility::Visible;
                }
            }
            polarity_indicator_board.polarity = plate.state;
        } else {
            *visible = Visibility::Visible;
        }
    }
}
