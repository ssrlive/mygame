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
        .add_systems(Update, update_bird)
        .run();
}

const PIXEL_RATIO: f32 = 4.0;
const FLAP_FORCE: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bird.png")),
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0.0 },
    ));
}

fn update_bird(
    mut bird_query: Query<(&mut Transform, &mut Bird)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
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
    }
}
