use bevy::{prelude::*, window::PrimaryWindow};
use rand::{rngs::ThreadRng, thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(800.0, 600.0).into(),
                        ..Window::default()
                    }),
                    ..WindowPlugin::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup_level)
        .add_systems(Update, (update_bird, update_obstacles))
        .run();
}

const PIXEL_RATIO: f32 = 4.0;
const FLAP_FORCE: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

const OBSTACLE_AMOUNT: i32 = 5;
const OBSTACLE_WIDTH: f32 = 32.0;
const OBSTACLE_HEIGHT: f32 = 144.0;
const OBSTACLE_VERTICAL_OFFSET: f32 = 30.0;
const OBSTACLE_GAP_SIZE: f32 = 15.0;
const OBSTACLE_SPACING: f32 = 60.0;
const OBSTACLE_SCROLL_SPEED: f32 = 150.0;

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pipe_image = asset_server.load("pipe.png");
    let window = window_query.get_single().unwrap();
    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bird.png")),
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0.0 },
    ));

    let mut rand = thread_rng();
    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image);
}

fn update_bird(
    mut commands: Commands,
    mut bird_query: Query<(&mut Transform, &mut Bird), Without<Obstacle>>,
    mut obstacle_query: Query<(&mut Transform, Entity), With<Obstacle>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    if let Ok((mut transform, mut bird)) = bird_query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }
        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90.0, 90.0).to_radians(),
        );

        let mut dead = false;
        if transform.translation.y <= -game_manager.window_dimensions.y / 2. {
            dead = true;
        } else {
            for (pipe_transform, _entity) in obstacle_query.iter_mut() {
                let pipe_tran = &pipe_transform.translation;
                let trans_trans = &transform.translation;
                if (pipe_tran.y - trans_trans.y).abs() < OBSTACLE_HEIGHT / 2.0 * PIXEL_RATIO
                    && (pipe_tran.x - trans_trans.x).abs() < OBSTACLE_WIDTH / 2.0 * PIXEL_RATIO
                {
                    dead = true;
                    break;
                }
            }
        }
        if dead {
            transform.translation = Vec3::ZERO;
            bird.velocity = 0.0;
            for (_transform, entity) in obstacle_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            let mut rand = thread_rng();
            spawn_obstacles(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image,
            );
        }
    }
}

fn update_obstacles(
    mut obstacle_query: Query<(&mut Transform, &mut Obstacle)>,
    time: Res<Time>,
    game_manager: Res<GameManager>,
) {
    let mut rand = thread_rng();
    let y_offset = generate_offset(&mut rand);
    for (mut transform, obstacle) in obstacle_query.iter_mut() {
        transform.translation.x -= OBSTACLE_SCROLL_SPEED * time.delta_secs();
        if transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2.
            < -game_manager.window_dimensions.x / 2.
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
            transform.translation.y =
                get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}

fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        let x_pos = window_width / 2. + OBSTACLE_SPACING * PIXEL_RATIO * i as f32;
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            1.0,
            commands,
            pipe_image,
        );
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            -1.0,
            commands,
            pipe_image,
        );
    }
}

fn get_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

#[derive(Component)]
pub struct Obstacle {
    pub pipe_direction: f32,
}

fn spawn_obstacle(
    translation: Vec3,
    // bottom or top of screen
    pipe_direction: f32,
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite::from_image(pipe_image.clone()),
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.gen_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}
