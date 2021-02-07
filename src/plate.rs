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

pub fn plate_control_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Plate>) {
    let mut plate_count = 1;
    let mut plate_selected = 1;
    for mut plate in query.iter_mut() {
        if plate.selected {
            if keyboard_input.just_pressed(KeyCode::Up) {
                plate.toggle()
            }
            if keyboard_input.just_pressed(KeyCode::Down) {
                plate.off()
            }
            plate_selected = plate.id;
        }
        plate_count = plate_count.max(plate.id);
        dbg!(&plate);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        plate_selected -= 1;
        if plate_selected < 1 {
            plate_selected = plate_count;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        plate_selected += 1;
        if plate_selected > plate_count {
            plate_selected = 1;
        }
    }
    for mut plate in query.iter_mut() {
        if plate_selected == plate.id {
            plate.selected = true;
        } else {
            plate.selected = false;
        }
    }
}
