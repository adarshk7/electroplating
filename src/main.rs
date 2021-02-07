mod electron;
mod plate;

use bevy::prelude::*;

use crate::electron::{electron_physics_system, Electron};
use crate::plate::{plate_control_system, Plate};

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;
const OUTER_WALL_THICKNESS: f32 = 1.0;
const ELECTRON_SIZE: f32 = 3.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Nokia 3310 Game Jam 3".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            scale_factor_override: Some(10.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex(COLOR_LIGHT).unwrap()))
        .add_startup_system(setup.system())
        .add_system(electron_physics_system.system())
        .add_system(plate_control_system.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_foreground = materials.add(Color::hex(COLOR_DARK).unwrap().into());

    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        // Electron
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(ELECTRON_SIZE, ELECTRON_SIZE)),
            ..Default::default()
        })
        .with(Electron::new(3.0))
        // Outer wall
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(-(WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz((WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            ..Default::default()
        })
        // Electric plates
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(
                (WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                0.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(1.0, 4.0)),
            ..Default::default()
        })
        .with(Plate::new(1))
        .spawn(SpriteBundle {
            material: material_foreground,
            transform: Transform::from_xyz(
                0.0,
                (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(4.0, 1.0)),
            ..Default::default()
        })
        .with(Plate::new(2));
}
