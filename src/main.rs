use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Centered Square".into(),
            resolution: (800., 600.).into(),
            resizable: true,
            ..default()
        }),
        ..default()
    }));
    app.add_systems(Startup, (spawn_camera, spawn_square));
    app.add_systems(Update, move_square);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct Velocity(Vec2);

fn spawn_square(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0), // Center
        Square {},
        Velocity(Vec2::new(200.0, 0.0)),
    ));
}

fn move_square(
    mut transform: Single<&mut Transform, With<Square>>,
    mut velocity: Single<&mut Velocity, With<Square>>,
    sprite: Single<&Sprite, With<Square>>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let window = windows.single().expect("No Window? Really?");
    let size = sprite.custom_size.expect("No Single Square found");

    let half_width = window.width() / 2.0;
    let square_half_size = size.x / 2.0;

    // Move the square
    transform.translation += velocity.0.extend(0.0) * time.delta_secs();

    // Check for collision with borders
    if transform.translation.x + square_half_size >= half_width {
        transform.translation.x = half_width - square_half_size;
        velocity.0.x *= -1.0;
    } else if transform.translation.x - square_half_size <= -half_width {
        transform.translation.x = -half_width + square_half_size;
        velocity.0.x *= -1.0;
    }
}

fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(1., 1., 1.), Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            scale: Vec2::new(120.0, 20.0).extend(1.0),
            ..default()
        },
    ));
}
