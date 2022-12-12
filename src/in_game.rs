use std::time::Duration;

use crate::{
    enemy::{
        enemy_hit_player_system, move_enemy_system, spawn_enemy_system, Enemy, EnemySpawnConfig,
        EnemyVelocity,
    },
    player::{
        move_player_system, player_input_system, spawn_player, Player, PlayerState, PlayerVelocity,
        Sidewalk,
    },
    GameAssets, GameState, COLOR_GRAY, COLOR_RED, SIDE_WALK, WIN_HEIGHT, WIN_WIDTH,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameBackground;

#[derive(Component)]
pub struct SideWalkTop;

#[derive(Component)]
pub struct SideWalkBottom;

#[derive(Component)]
pub struct LevelCounter;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerVelocity>()
            .register_type::<Enemy>()
            .register_type::<EnemyVelocity>()
            .insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(in_game_setup_system)
                    .with_system(spawn_player),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(move_player_system)
                    .with_system(player_input_system)
                    .with_system(spawn_enemy_system)
                    .with_system(move_enemy_system)
                    .with_system(enemy_hit_player_system)
                    .with_system(sidewalk_hit_system)
                    .with_system(increment_level_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InGame).with_system(despawn_in_game_system),
            );
    }
}

fn in_game_setup_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut player_state: ResMut<PlayerState>,
) {
    player_state.alive = true;
    player_state.level = 1;
    player_state.last_sidewalk = Sidewalk::None;

    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });

    commands
        .spawn(SpriteBundle {
            texture: game_assets.road.clone(),
            ..default()
        })
        .insert(GameBackground)
        .insert(Name::new("Road"));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(COLOR_GRAY.0, COLOR_GRAY.1, COLOR_GRAY.2),
                custom_size: Some(Vec2::new(WIN_WIDTH, SIDE_WALK)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: -(WIN_HEIGHT + SIDE_WALK) / 2.,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        })
        .insert(SideWalkBottom)
        .insert(GameBackground)
        .insert(Name::new("SideWalkBottom"));

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(COLOR_GRAY.0, COLOR_GRAY.1, COLOR_GRAY.2),
                custom_size: Some(Vec2::new(WIN_WIDTH, SIDE_WALK)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: (WIN_HEIGHT + SIDE_WALK) / 2.,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        })
        .insert(SideWalkTop)
        .insert(GameBackground)
        .insert(Name::new("SideWalkTop"))
        .with_children(|parent| {
            parent
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "Lv: 1",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 35.,
                            color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2),
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new((WIN_WIDTH / 2.) - 100., SIDE_WALK / 2., 2.),
                        ..default()
                    },
                    ..default()
                })
                .insert(LevelCounter);
        });
}

fn sidewalk_hit_system(
    mut player_state: ResMut<PlayerState>,
    player_query: Query<&Transform, With<Player>>,
    mut spawn_timer: ResMut<EnemySpawnConfig>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        let player_y_pos = player_tf.translation.y;
        let next_level = player_state.level + 1;
        let next_time = 2. - f32::from(next_level) * 0.05;

        if player_state.last_sidewalk == Sidewalk::Bottom
            || player_state.last_sidewalk == Sidewalk::None
        {
            if player_y_pos > 300. && player_y_pos < 400. {
                player_state.last_sidewalk = Sidewalk::Top;
                player_state.level = next_level;

                spawn_timer
                    .timer
                    .set_duration(Duration::from_secs_f32(next_time));
            }
        }

        if player_state.last_sidewalk == Sidewalk::Top {
            if player_y_pos < -300. && player_y_pos > -400. {
                player_state.last_sidewalk = Sidewalk::Bottom;
                player_state.level = next_level;

                spawn_timer
                    .timer
                    .set_duration(Duration::from_secs_f32(next_time));
            }
        }
    }
}

fn increment_level_system(
    player_state: Res<PlayerState>,
    mut level_query: Query<&mut Text, With<LevelCounter>>,
) {
    if let Ok(mut level_text) = level_query.get_single_mut() {
        level_text.sections[0].value = format!("Lv: {}", player_state.level);
    }
}

fn despawn_in_game_system(
    mut commands: Commands,
    background_query: Query<Entity, With<GameBackground>>,
    level_query: Query<Entity, With<LevelCounter>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for ent in enemy_query.iter() {
        commands.entity(ent).despawn();
    }

    for ent in background_query.iter() {
        commands.entity(ent).despawn();
    }

    if let Ok(level_ent) = level_query.get_single() {
        commands.entity(level_ent).despawn_recursive();
    }
}
