use crate::{
    enemy::{
        enemy_hit_player_system, move_enemy_system, setup_spawn_enemy_system, spawn_enemy_system,
        Enemy, EnemyVelocity,
    },
    player::{move_player_system, player_input_system, spawn_player, PlayerState, PlayerVelocity},
    GameState, GameTextures, COLOR_GRAY, SIDE_WALK, WIN_HEIGHT, WIN_WIDTH,
};
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameBackground;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SideWalkTop;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SideWalkBottom;

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
                    .with_system(spawn_player)
                    .with_system(setup_spawn_enemy_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(move_player_system)
                    .with_system(player_input_system)
                    .with_system(spawn_enemy_system)
                    .with_system(move_enemy_system)
                    .with_system(enemy_hit_player_system),
            );
    }
}

fn in_game_setup_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    player_state.alive = true;
    player_state.level = 1;

    commands
        .spawn(SpriteBundle {
            texture: game_textures.road.clone(),
            ..default()
        })
        .insert(Name::new("GameBackground"));

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
        .insert(Name::new("SideWalkTop"));
}
