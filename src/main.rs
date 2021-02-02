use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;

// const COLOR_LIGHT: (f32, f32, f32) = (0.78, 0.941, 0.847);
// const COLOR_DARK: (f32, f32, f32) = (0.263, 0.322, 0.239);

struct Electron {
    speed: f32,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Nokia 3310 Game Jam 3".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.78, 0.941, 0.847)))
        .add_startup_system(setup.system())
        // .add_system(electron_physics.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.263, 0.322, 0.239).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(2.0, 2.0)),
            ..Default::default()
        })
        .with(Electron { speed: 2.0 });

    let window = windows.get_primary_mut().unwrap();
    window.set_scale_factor_override(Some(10.0));
}

// fn electron_physics(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Electron, &mut Transform)>,
// ) {
//     for (electron, mut transform) in query.iter_mut() {
//         let mut direction = Vec2::new(0.0, 0.0);
//         if keyboard_input.pressed(KeyCode::Up) {
//             direction = direction + Vec2::new(0.0, -1.0);
//         }
//         if keyboard_input.pressed(KeyCode::Down) {
//             direction = direction + Vec2::new(0.0, 1.0);
//         }
//         if keyboard_input.pressed(KeyCode::Left) {
//             direction = direction + Vec2::new(0.0, -1.0);
//         }
//         if keyboard_input.pressed(KeyCode::Right) {
//             direction = direction + Vec2::new(0.0, 1.0);
//         }
//         direction = direction.normalize();
//         let translation = &mut transform.translation;
//         translation.x += time.delta_seconds() * direction.x * electron.speed;
//         translation.y += time.delta_seconds() * direction.y * electron.speed;
//         dbg!(translation);
//     }
// }
