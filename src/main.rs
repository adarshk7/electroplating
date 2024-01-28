mod asset_loader;
mod electron;
mod plate;

use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;

use crate::asset_loader::AssetLoaderPlugin;
use crate::electron::{electron_physics_system, Electron};
use crate::plate::PlateSelectedAnimationTimer;
use crate::plate::{
    plate_control_system, polarity_indicator_board_system, Plate, PlateState,
    PolarityIndicatorBoard,
};

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;
const OUTER_WALL_THICKNESS: f32 = 1.0;
const ELECTRON_SIZE: f32 = 3.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

const FONT_SIZE: f32 = 4.0;
const TEXT_POSITION_TOP: f32 = 1.0;
const TEXT_POSITION_LEFT: f32 = 2.0;

const PLATE_ANIMATION_TIMER_PERIOD: f32 = 0.25;

fn main() {
    let mut app = App::new();

    app.insert_resource(PolarityIndicatorBoard {
        polarity: PlateState::Negative,
    });
    app.insert_resource(PlateSelectedAnimationTimer {
        timer: Timer::from_seconds(PLATE_ANIMATION_TIMER_PERIOD, TimerMode::Repeating),
    });
    app.insert_resource(ClearColor(Color::hex(COLOR_LIGHT).unwrap()));

    let mut resolution = WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    resolution.set_scale_factor_override(Some(10.0));

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Electroplating".into(),
                    resolution,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        AssetLoaderPlugin,
    ));
    app.add_systems(Startup, setup);
    app.add_systems(Update, electron_physics_system);
    app.add_systems(Update, plate_control_system);
    app.add_systems(Update, polarity_indicator_board_system);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_plate_off = asset_server.load("textures/plate_off.png");
    let texture_exit = asset_server.load("textures/exit.png");
    let music: Handle<AudioSource> = asset_server.load("sound/bad_melody.wav");

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);
    commands.spawn(camera);

    commands.spawn(AudioSourceBundle {
        source: music,
        settings: PlaybackSettings {
            mode: PlaybackMode::Once,
            ..default()
        },
    });
    // Electron
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ELECTRON_SIZE, ELECTRON_SIZE)),
                color: Color::hex(COLOR_DARK).unwrap(),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        Electron::new(10.0, Vec3::new(-35.0, 15.0, 0.0)),
    ));
    // Outer wall
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-(WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            color: Color::hex(COLOR_DARK).unwrap(),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz((WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            color: Color::hex(COLOR_DARK).unwrap(),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            color: Color::hex(COLOR_DARK).unwrap(),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            color: Color::hex(COLOR_DARK).unwrap(),
            ..default()
        },
        ..default()
    });
    // Middle wall
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-20.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH - 20.0, 2.0)),
            color: Color::hex(COLOR_DARK).unwrap(),
            ..default()
        },
        ..default()
    });
    // Exit
    commands.spawn(SpriteBundle {
        texture: texture_exit,
        transform: Transform::from_xyz(-35.0, -15.0, 0.0),
        ..default()
    });
    // Electric plates
    commands.spawn((
        SpriteBundle {
            texture: texture_plate_off.clone(),
            transform: Transform::from_xyz(
                (WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                -7.0,
                0.0,
            ),
            visibility: Visibility::Visible,
            ..default()
        },
        Plate::new(1),
    ));
    commands.spawn((
        SpriteBundle {
            texture: texture_plate_off,
            transform: Transform {
                translation: Vec3::new(
                    10.0,
                    (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        Plate::new(2),
    ));
    // UI text
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "POLARITY ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("font/EffortsPro.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::hex(COLOR_DARK).unwrap(),
                    },
                },
                TextSection {
                    value: "OFF".to_string(),
                    style: TextStyle {
                        font: asset_server.load("font/EffortsPro.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::hex(COLOR_DARK).unwrap(),
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(TEXT_POSITION_TOP),
            left: Val::Px(TEXT_POSITION_LEFT),
            ..default()
        },
        ..default()
    });
}
