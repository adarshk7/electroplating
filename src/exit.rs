use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{electron::Electron, AppState};

#[derive(Component)]
pub struct Exit;

pub fn handle_exit(
    rapier_context: Res<RapierContext>,
    exit_query: Query<Entity, With<Exit>>,
    electron_query: Query<Entity, With<Electron>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(exit) = exit_query.get_single() else {
        return;
    };
    let Ok(electron_entity) = electron_query.get_single() else {
        return;
    };
    if rapier_context.intersection_pair(electron_entity, exit) == Some(true) {
        next_state.set(AppState::Victory);
    }
}
