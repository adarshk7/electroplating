use bevy::prelude::*;
#[derive(Debug)]
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

#[derive(Debug)]
pub enum PlateState {
    Positive,
    Negative,
    Off,
}

pub struct PlateSelectedAnimationTimer {
    pub timer: Timer,
}

pub fn plate_control_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Plate, &mut Visible)>,
    mut animation_timer: ResMut<PlateSelectedAnimationTimer>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut plate_count = 1;
    let mut plate_selected = 1;
    for (mut plate, _) in query.iter_mut() {
        if plate.selected {
            if keyboard_input.just_pressed(KeyCode::Up) {
                let music = asset_server.load("sound/blip5.wav");
                audio.play(music);
                plate.toggle();
            }
            if keyboard_input.just_pressed(KeyCode::Down) {
                let music = asset_server.load("sound/blip3.wav");
                audio.play(music);
                plate.off();
            }
            plate_selected = plate.id;
        }
        plate_count = plate_count.max(plate.id);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        let music = asset_server.load("sound/blip8.wav");
        audio.play(music);
        plate_selected -= 1;
        if plate_selected < 1 {
            plate_selected = plate_count;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        let music = asset_server.load("sound/blip9.wav");
        audio.play(music);
        plate_selected += 1;
        if plate_selected > plate_count {
            plate_selected = 1;
        }
    }
    for (mut plate, mut visible) in query.iter_mut() {
        plate.selected = plate_selected == plate.id;
        if plate.selected
            && animation_timer
                .timer
                .tick(time.delta_seconds())
                .just_finished()
        {
            visible.is_visible = !visible.is_visible;
        }
        if !plate.selected {
            visible.is_visible = true;
        }
    }
}
