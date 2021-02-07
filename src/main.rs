use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;
const OUTER_WALL_THICKNESS: f32 = 1.0;
const ELECTRON_SIZE: f32 = 3.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

#[derive(Debug)]
struct Electron {}

#[derive(Debug)]
struct Plate {
    id: usize,
    state: PlateState,
    selected: bool,
}

impl Plate {
    fn new(id: usize) -> Self {
        Plate {
            id,
            state: PlateState::Off,
            selected: false,
        }
    }

    fn toggle(&mut self) {
        self.state = match self.state {
            PlateState::Positive => PlateState::Negative,
            PlateState::Negative => PlateState::Positive,
            PlateState::Off => PlateState::Positive,
        }
    }

    fn off(&mut self) {
        self.state = PlateState::Off
    }
}

#[derive(Debug)]
enum PlateState {
    Positive,
    Negative,
    Off,
}

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
        .add_system(electron_physics.system())
        .add_system(plate_control.system())
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
        .with(Electron {})
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

fn electron_physics(
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

fn plate_control(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Plate>) {
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
