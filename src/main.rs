//------------------------------------------------------------------------------
// Snek
//------------------------------------------------------------------------------

use bevy::prelude::*;

mod direction;
use direction::Direction;

mod constants;
use constants::*;

// Components
//------------------------------------------------------------------------------
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct SpriteSize {
    width: i32,
    height: i32,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
    next_direction: Direction,
}

// References
//------------------------------------------------------------------------------
#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

// Startup systems
//------------------------------------------------------------------------------
fn init_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn init_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SnakeHead {
            direction: SNAKE_INITIAL_DIRECTION,
            next_direction: SNAKE_INITIAL_DIRECTION,
        },
        Position {
            x: 0, y: 0,
        },
        Size::square(1.),
        SpriteSize {
            width: 128, height: 128,
        },
        SpriteBundle {
            texture: asset_server.load("snake_head.png"),
            ..Default::default()
        },
    ));
}

// FixedUpdate systems
//------------------------------------------------------------------------------
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut SnakeHead>,
) {
    let mut head = q.single_mut();

    let direction: Direction =
        if keyboard_input.pressed(KeyCode::KeyW) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            Direction::Right
        } else {
            head.direction
        };

    if direction != head.direction
    && direction != head.direction.opposite() {
        head.next_direction = direction;
    }
}

// TODO: Fix 1/2 tile offset.
fn move_snake(mut q: Query<(&mut SnakeHead, &mut Position, &mut Transform)>) {
    fn pixel_to_tile(
        translation: f32,
        boundary_pixels: f32,
        boundary_tiles: f32
    ) -> i32 {
        ((translation - (boundary_pixels / boundary_tiles * 0.5)) / boundary_pixels * boundary_tiles) as i32
    }

    let (mut head, mut position, mut transform) = q.single_mut();

    transform.translation.x += head.direction.vector().x * SNAKE_SPEED;
    transform.translation.y += head.direction.vector().y * SNAKE_SPEED;

    let actual_position: Position = Position {
        x: pixel_to_tile(transform.translation.x, LEVEL_WIDTH as f32, LEVEL_TILES_X as f32),
        y: pixel_to_tile(transform.translation.y, LEVEL_HEIGHT as f32, LEVEL_TILES_Y as f32),
    };

    if *position != actual_position {
        *position = actual_position;

        if head.direction != head.next_direction {
            head.direction = head.next_direction;
        }
    }

    info!("x {}, y {}", position.x, position.y)
}

// Update systems
//------------------------------------------------------------------------------
fn apply_size(mut q: Query<(&Size, &SpriteSize, &mut Transform)>) {
    for (size, sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            (LEVEL_WIDTH as f32 / LEVEL_TILES_X as f32) / sprite_size.width as f32 * size.width,
            (LEVEL_HEIGHT as f32 / LEVEL_TILES_Y as f32) / sprite_size.height as f32 * size.height,
            1.,
        );
    }
}

// Entry point
//------------------------------------------------------------------------------
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snek".into(),
                        resolution: (800., 600.).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(
            Startup, (
                init_camera,
                init_snake,
            ).chain(),
        )
        .add_systems(
            FixedUpdate, (
                handle_input,
                move_snake,
            ).chain(),
        )
        .add_systems(Update,
            bevy::window::close_on_esc,
        )
        .add_systems(
            PostUpdate, (
                apply_size,
            ).chain(),
        )
        .run();
}
