use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const BALL_RADIUS: f32 = 25.;
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
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..RapierConfiguration::new(1.)
    });

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(
        Startup,
        (spawn_camera, spawn_players, spawn_ball, spawn_border),
    );
    app.add_systems(Update, (move_paddle,));

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

#[derive(Component)]
enum Player {
    Player1,
    Player2,
}

fn spawn_border(mut commands: Commands) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., WINDOW_HEIGHT / 2., 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.),
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., -WINDOW_HEIGHT / 2., 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2., 3.),
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HEIGHT / 2.),
        Player::Player1,
        Sensor,
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(3., WINDOW_HEIGHT / 2.),
        Player::Player2,
        Sensor,
    ));
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
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5., 75.),
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
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5., 75.),
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
struct Ball;

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_RADIUS, BALL_RADIUS)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball,
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::new(-350., 0.)),
        Restitution {
            coefficient: 1.1,
            combine_rule: CoefficientCombineRule::Max,
        },
        // LockedAxes::ROTATION_LOCKED,
    ));
}
