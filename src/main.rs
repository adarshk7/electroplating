use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

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
    let wall_thickness = 1.0;

    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        // Electron
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(2.0, 2.0)),
            ..Default::default()
        })
        .with(Electron {})
        // Outer wall
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(-(WINDOW_WIDTH - wall_thickness) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz((WINDOW_WIDTH - wall_thickness) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT - wall_thickness) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, wall_thickness)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground,
            transform: Transform::from_xyz(0.0, (WINDOW_HEIGHT - wall_thickness) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, wall_thickness)),
            ..Default::default()
        });
}

fn electron_physics(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Electron)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Up) {
            transform.translation.y += 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
    }
}
