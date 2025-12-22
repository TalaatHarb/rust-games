use bevy::prelude::*;
use rand::Rng;

pub const SPRITE_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_MAX_COUNT: usize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, player_movement)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, enemy_update_direction)
            .add_systems(Update, confine_movement);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {},
        Confined {},
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.single() {
        let mut rng = rand::rng();
        
        // Calculate spawn boundaries (keep enemies away from edges)
        let half_enemy_size = SPRITE_SIZE / 2.0;
        let x_min = -window.width() / 2.0 + half_enemy_size;
        let x_max = window.width() / 2.0 - half_enemy_size;
        let y_min = -window.height() / 2.0 + half_enemy_size;
        let y_max = window.height() / 2.0 - half_enemy_size;
        
        // Spawn enemies at random positions
        for _ in 0..ENEMY_MAX_COUNT {
            let x = rng.random_range(x_min..x_max);
            let y = rng.random_range(y_min..y_max);
            
            commands.spawn((
                Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
                Transform::from_xyz(x, y, 0.0),
                Enemy {
                    direction: Vec3::new(
                        rng.random_range(-1.0..1.0),
                        rng.random_range(-1.0..1.0),
                        0.0,
                    ).normalize(),
                },
                Confined {},
            ));
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * dt;
    }
}

pub fn enemy_update_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.single() {
        let half_sprite_size = SPRITE_SIZE / 2.0;
        let x_min = -window.width() / 2.0 + half_sprite_size;
        let x_max = window.width() / 2.0 - half_sprite_size;
        let y_min = -window.height() / 2.0 + half_sprite_size;
        let y_max = window.height() / 2.0 - half_sprite_size;

        for (transform, mut enemy) in enemy_query.iter_mut() {
            let mut dir = enemy.direction;

            if transform.translation.x <= x_min && dir.x < 0.0 {
                dir.x = -dir.x;
            } else if transform.translation.x >= x_max && dir.x > 0.0 {
                dir.x = -dir.x;
            }

            if transform.translation.y <= y_min && dir.y < 0.0 {
                dir.y = -dir.y;
            } else if transform.translation.y >= y_max && dir.y > 0.0 {
                dir.y = -dir.y;
            }

            if dir.length() > 0.0 {
                enemy.direction = dir.normalize();
            } else {
                enemy.direction = Vec3::ZERO;
            }
        }
    }
}

pub fn confine_movement(
    mut confined_query: Query<&mut Transform, With<Confined>>,
    window_query: Query<&Window>,
) {
    if let Ok(window) = window_query.single() {
        let half_sprite_size = SPRITE_SIZE / 2.0;
        let x_min = -window.width() / 2.0 + half_sprite_size;
        let x_max = window.width() / 2.0 - half_sprite_size;
        let y_min = -window.height() / 2.0 + half_sprite_size;
        let y_max = window.height() / 2.0 - half_sprite_size;

        for mut transform in confined_query.iter_mut() {
            transform.translation.x = transform.translation.x.clamp(x_min, x_max);
            transform.translation.y = transform.translation.y.clamp(y_min, y_max);
        }
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3
}

#[derive(Component)]
pub struct Confined{}