use crate::{
    player::{Player, PlayerState},
    GameState, GameTextures, BASE_SPEED, PLAYER_DIM, SIDE_WALK, TIME_STEP, WIN_WIDTH,
};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};
use std::time::Duration;

const FIRST_LANE: f32 = -210.;
const SECOND_LANE: f32 = 0.;
const THIRD_LANE: f32 = 198.;

const ENEMY_DIM: (f32, f32) = (48., 24.);
const ENEMY_SCALE: f32 = 2.5;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyVelocity {
    pub x: f32,
}

#[derive(Resource, Default)]
struct EnemySpawnConfig {
    timer: Timer,
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyVelocity>()
            .add_system_set(
                SystemSet::on_enter(GameState::InGame).with_system(setup_spawn_enemy_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(spawn_enemy_system)
                    .with_system(move_enemy_system)
                    .with_system(enemy_hit_player_system),
            );
    }
}

fn setup_spawn_enemy_system(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
}

fn spawn_enemy_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut spawn_timer: ResMut<EnemySpawnConfig>,
    time: Res<Time>,
    player_state: Res<PlayerState>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.finished() && player_state.alive {
        let mut rng = thread_rng();
        let lane_index: usize = rng.gen_range(0..3);
        let lanes = [FIRST_LANE, SECOND_LANE, THIRD_LANE];

        let car_color: u8 = rng.gen_range(0..2);
        let car_sprite = if car_color == 0 {
            game_textures.enemy_red.clone()
        } else {
            game_textures.enemy_green.clone()
        };

        commands
            .spawn(SpriteBundle {
                texture: car_sprite,
                transform: Transform {
                    translation: Vec3 {
                        x: (-WIN_WIDTH / 2.) + (ENEMY_DIM.0 * ENEMY_SCALE) - 50.,
                        y: lanes[lane_index],
                        z: 2.,
                    },
                    scale: Vec3 {
                        x: 2.5,
                        y: 2.5,
                        z: 2.5,
                    },
                    ..default()
                },
                ..default()
            })
            .insert(EnemyVelocity { x: 1. })
            .insert(Enemy)
            .insert(Name::new("Enemy"));
    }
}

fn move_enemy_system(
    mut commands: Commands,
    mut enemies_query: Query<(Entity, &mut Transform, &EnemyVelocity), With<Enemy>>,
    player_state: Res<PlayerState>,
) {
    for (enemy_entity, mut transform, velocity) in enemies_query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        if transform.translation.x > WIN_WIDTH + ENEMY_DIM.0 || !player_state.alive {
            commands.entity(enemy_entity).despawn();
        }
    }
}

fn enemy_hit_player_system(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut EnemyVelocity), With<Enemy>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut player_state: ResMut<PlayerState>,
) {
    if let Ok((player_ent, player_tf)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());
        for (enemy_tf, mut enemy_vel) in enemy_query.iter_mut() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                player_tf.translation,
                PLAYER_DIM * player_scale,
                enemy_tf.translation,
                ENEMY_DIM.0 * enemy_scale,
            );

            if let Some(_) = collision {
                player_state.alive = false;
                commands.entity(player_ent).despawn();
            }
        }
    }
}
