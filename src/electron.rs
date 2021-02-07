use bevy::prelude::*;

use crate::{
    plate::{Plate, PlateState},
    ELECTRON_SIZE, OUTER_WALL_THICKNESS, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const INVERSE_SQUARE_ATTRACTION_FACTOR: f32 = 20.0;

#[derive(Debug)]
pub struct Electron {
    pub location: Vec3,
    pub speed: f32,
}

impl Electron {
    pub fn new(speed: f32) -> Self {
        Self {
            location: Vec3::new(0.0, 0.0, 0.0),
            speed,
        }
    }
}

pub fn electron_physics_system(
    time: Res<Time>,
    mut query_electron: Query<(&mut Transform, &mut Electron)>,
    query_plates: Query<(&Transform, &Plate)>,
) {
    for (mut electron_transform, mut electron) in query_electron.iter_mut() {
        let electron_loc = &mut electron_transform.translation;
        let mut components: Vec<Vec3> = Vec::new();
        for (plate_transform, plate) in query_plates.iter() {
            let plate_loc = &plate_transform.translation;
            let magnitude_inverse_squared = 1.0 / (*plate_loc - *electron_loc).length_squared();
            let direction = *plate_loc - *electron_loc;
            direction.normalize();
            let force_component_vector = match plate.state {
                PlateState::Negative => {
                    INVERSE_SQUARE_ATTRACTION_FACTOR * direction * -1.0 * magnitude_inverse_squared
                }
                PlateState::Positive => {
                    INVERSE_SQUARE_ATTRACTION_FACTOR * direction * 1.0 * magnitude_inverse_squared
                }
                PlateState::Off => direction * 0.0,
            };
            components.push(force_component_vector);
        }
        let force = components.iter().sum::<Vec3>();
        force.normalize();
        let delta_pixel: Vec3 = force * time.delta_seconds() * electron.speed;
        electron.location += delta_pixel;
        electron_loc.x = electron.location.x.round();
        electron_loc.y = electron.location.y.round();
        electron_bounds(&mut electron.location);
        dbg!(&electron);
    }
}

/// Set electron bounds with outer walls.
fn electron_bounds(translation: &mut Vec3) {
    translation.y = translation
        .y
        .min((WINDOW_HEIGHT / 2.0) - OUTER_WALL_THICKNESS - ELECTRON_SIZE / 2.0);
    translation.y = translation
        .y
        .max((-WINDOW_HEIGHT / 2.0) + OUTER_WALL_THICKNESS + ELECTRON_SIZE / 2.0);
    translation.x = translation
        .x
        .min((WINDOW_WIDTH / 2.0) - OUTER_WALL_THICKNESS - ELECTRON_SIZE / 2.0);
    translation.x = translation
        .x
        .max((-WINDOW_WIDTH / 2.0) + OUTER_WALL_THICKNESS + ELECTRON_SIZE / 2.0);
}
