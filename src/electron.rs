use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plate::{Plate, PlateState};

const INVERSE_SQUARE_ATTRACTION_FACTOR: f32 = 20.0;

#[derive(Debug, Component)]
pub struct Electron {
    pub speed: f32,
}

impl Electron {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

pub fn electron_physics_system(
    time: Res<Time>,
    mut query_electron: Query<(&mut ExternalForce, &mut Electron, &Transform), Without<Plate>>,
    query_plates: Query<(&Transform, &Plate)>,
) {
    let Ok((mut ext_force, electron, electron_transform)) = query_electron.get_single_mut() else {
        return;
    };
    let electron_loc = &mut electron_transform.translation.xy();
    let mut components = Vec::new();
    for (plate_transform, plate) in &query_plates {
        let plate_loc = &plate_transform.translation.xy();
        let magnitude_inverse_squared =
            INVERSE_SQUARE_ATTRACTION_FACTOR / (*plate_loc - *electron_loc).length_squared();
        let direction = *plate_loc - *electron_loc;
        let force_component_vector = match plate.state {
            PlateState::Negative => direction * -1.0 * magnitude_inverse_squared,
            PlateState::Positive => direction * 1.0 * magnitude_inverse_squared,
            PlateState::Off => direction * 0.0,
        };
        components.push(force_component_vector);
    }
    let force = components.iter().sum::<Vec2>();
    let delta_pixel = force * time.delta_seconds() * electron.speed;
    ext_force.force = delta_pixel;
}
