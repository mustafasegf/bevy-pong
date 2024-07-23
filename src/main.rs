use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;

const BALL_WIDTH: f32 = 25.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 150.;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    app.add_systems(Update, (move_paddle, move_ball, ball_collide));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2. + 20., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2. - 20., 0., 0.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 200. * time.delta_seconds();
            pos.translation.y = pos
                .translation
                .y
                .clamp(-WINDOW_HEIGHT / 2. + 75., WINDOW_HEIGHT / 2. - 75.);
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= 200. * time.delta_seconds();
            pos.translation.y = pos
                .translation
                .y
                .clamp(-WINDOW_HEIGHT / 2. + 75., WINDOW_HEIGHT / 2. - 75.);
        }
    }
}

#[derive(Component)]
struct Ball(Vec2);

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_WIDTH, BALL_WIDTH)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball(Vec2::new(-100., 0.)),
    ));
}

fn move_ball(mut ball: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut ball {
        pos.translation += 2. * ball.0.extend(0.) * time.delta_seconds();
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        if ball.translation.y.abs() + BALL_WIDTH / 2. > 250. {
            velocity.0.y *= -1.;
        }
        for paddle in &paddles {
            if ball.translation.x - BALL_WIDTH / 2. < paddle.translation.x + PADDLE_WIDTH / 2.
                && ball.translation.y - BALL_WIDTH / 2. < paddle.translation.y + PADDLE_HEIGHT / 2.
                && ball.translation.x + BALL_WIDTH / 2. > paddle.translation.x - PADDLE_WIDTH / 2.
                && ball.translation.y + BALL_WIDTH / 2. > paddle.translation.y - PADDLE_HEIGHT / 2.
            {
                velocity.0 *= -1.;
                velocity.0.y = rand::thread_rng().gen_range(-1.0..1.0) * 100.;
            }
        }
    }
}
