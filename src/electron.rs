use bevy::prelude::*;

use crate::{
    plate::{Plate, PlateState},
    ELECTRON_SIZE, OUTER_WALL_THICKNESS, WINDOW_HEIGHT, WINDOW_WIDTH,
};
#[derive(Debug)]
pub struct Electron {}

pub fn electron_physics_system(
    mut query_electron: Query<(&mut Transform, &Electron)>,
    query_plates: Query<(&Transform, &Plate)>,
) {
    for (mut electron_transform, _) in query_electron.iter_mut() {
        let electron_loc = &mut electron_transform.translation;
        for (plate_transform, plate) in query_plates.iter() {
            let plate_loc = &plate_transform.translation;
            match plate.state {
                PlateState::Negative => {
                    if electron_loc.x < plate_loc.x {
                        electron_loc.x -= 1.0;
                    } else if electron_loc.x > plate_loc.x {
                        electron_loc.x += 1.0;
                    }
                    if electron_loc.y < plate_loc.y {
                        electron_loc.y -= 1.0;
                    } else if electron_loc.y > plate_loc.y {
                        electron_loc.y += 1.0;
                    }
                }
                PlateState::Positive => {
                    if electron_loc.x < plate_loc.x {
                        electron_loc.x += 1.0;
                    } else if electron_loc.x > plate_loc.x {
                        electron_loc.x -= 1.0;
                    }
                    if electron_loc.y < plate_loc.y {
                        electron_loc.y += 1.0;
                    } else if electron_loc.y > plate_loc.y {
                        electron_loc.y -= 1.0;
                    }
                }
                PlateState::Off => (),
            }
        }
        electron_bounds(&mut electron_transform);
    }
}

/// Set electron bounds with outer walls.
fn electron_bounds(transform: &mut Transform) {
    transform.translation.y = transform
        .translation
        .y
        .min((WINDOW_HEIGHT / 2.0) - OUTER_WALL_THICKNESS - ELECTRON_SIZE / 2.0);
    transform.translation.y = transform
        .translation
        .y
        .max((-WINDOW_HEIGHT / 2.0) + OUTER_WALL_THICKNESS + ELECTRON_SIZE / 2.0);
    transform.translation.x = transform
        .translation
        .x
        .min((WINDOW_WIDTH / 2.0) - OUTER_WALL_THICKNESS - ELECTRON_SIZE / 2.0);
    transform.translation.x = transform
        .translation
        .x
        .max((-WINDOW_WIDTH / 2.0) + OUTER_WALL_THICKNESS + ELECTRON_SIZE / 2.0);
}
