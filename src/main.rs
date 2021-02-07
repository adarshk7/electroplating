use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;
const OUTER_WALL_THICKNESS: f32 = 1.0;
const ELECTRON_SIZE: f32 = 3.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

#[derive(Debug)]
struct Electron {}

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
            material: material_foreground,
            transform: Transform::from_xyz(0.0, (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            ..Default::default()
        });
}

fn electron_physics(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Electron)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
        // Set bounds with outer walls
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
}
