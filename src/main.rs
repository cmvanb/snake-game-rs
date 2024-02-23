use bevy::prelude::*;

// Constants
const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
const UP: Vec2 = Vec2::new(0.0, 1.0);
const DOWN: Vec2 = Vec2::new(0.0, -1.0);

const SNAKE_SPEED: f32 = 200.0;

// Components
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct SnakeHead;

// Setup system
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SnakeHead,
        SpriteBundle {
            texture: asset_server.load("snake_head.png"),
            transform: Transform {
                translation: Vec3::new(0.0,  0.0,  0.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..default()
            },
            ..Default::default()
        },
        Velocity(RIGHT.normalize() * SNAKE_SPEED),
    ));
}

// Gameplay systems
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn control_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<SnakeHead>>,
) {
    let mut velocity = query.single_mut();
    let mut direction: Vec2 = Vec2::new(velocity.x, velocity.y).normalize();

    if keyboard_input.pressed(KeyCode::W) {
        direction = UP;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction = DOWN;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction = LEFT;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction = RIGHT;
    }

    velocity.x = direction.x * SNAKE_SPEED;
    velocity.y = direction.y * SNAKE_SPEED;
}

// Application
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            control_snake,
            apply_velocity,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
