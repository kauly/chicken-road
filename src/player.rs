use crate::{
    spawn_game_background, GameState, GameTextures, BASE_SPEED, SIDE_WALK, TIME_STEP, WIN_HEIGHT,
    WIN_WIDTH,
};
use bevy::prelude::*;

const PLAYER_RIGHT_SPRITE_INDEX: (usize, usize) = (0, 5);
const PLAYER_UP_SPRITE_INDEX: (usize, usize) = (6, 11);
const PLAYER_LEFT_SPRITE_INDEX: (usize, usize) = (12, 17);
const PLAYER_DOWN_SPRITE_INDEX: (usize, usize) = (18, 23);
const PLAYER_DIM: f32 = 16.;

fn get_sprite_index(dim: (usize, usize), current_index: usize) -> usize {
    println!("dim: {:?}, index: {}", dim, current_index);
    let index = if current_index >= dim.0 && current_index < dim.1 {
        current_index + 1
    } else {
        dim.0
    };
    return index;
}

#[derive(Resource)]
pub struct PlayerState {
    alive: bool,
    level: u8,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: false,
            level: 1,
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerVelocity>()
            .register_type::<Direction>()
            .insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(spawn_player)
                    .with_system(move_player_system)
                    .with_system(player_input_system)
                    .with_system(spawn_game_background),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut player_state: ResMut<PlayerState>,
) {
    if !player_state.alive {
        player_state.alive = true;
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.player.clone(),
                sprite: TextureAtlasSprite {
                    index: PLAYER_UP_SPRITE_INDEX.0,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: (-WIN_HEIGHT - SIDE_WALK + PLAYER_DIM) / 2.,
                        z: 1.,
                    },
                    scale: Vec3 {
                        x: 2.,
                        y: 2.,
                        z: 2.,
                    },
                    ..default()
                },
                ..default()
            })
            .insert(PlayerVelocity::default())
            .insert(Player)
            .insert(Direction::Up)
            .insert(Name::new("Player"));
    }
}

fn move_player_system(
    mut player_query: Query<
        (
            &mut Transform,
            &PlayerVelocity,
            &mut TextureAtlasSprite,
            &Direction,
        ),
        With<Player>,
    >,
    //time: Res<Time>,
) {
    if let Ok((mut transform, velocity, mut sprite, direction)) = player_query.get_single_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if velocity.x != 0. || velocity.y != 0. {
            sprite.index = match direction {
                Direction::Up => get_sprite_index(PLAYER_UP_SPRITE_INDEX, sprite.index),
                Direction::Down => get_sprite_index(PLAYER_DOWN_SPRITE_INDEX, sprite.index),
                Direction::Left => get_sprite_index(PLAYER_LEFT_SPRITE_INDEX, sprite.index),
                Direction::Right => get_sprite_index(PLAYER_RIGHT_SPRITE_INDEX, sprite.index),
            }
        }
    }
}

fn player_input_system(
    kb: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerVelocity, &mut Direction), With<Player>>,
) {
    if let Ok((mut velocity, mut direction)) = player_query.get_single_mut() {
        if kb.pressed(KeyCode::A) {
            velocity.x = -1.;
            velocity.y = 0.;
            *direction = Direction::Left;
        } else if kb.pressed(KeyCode::D) {
            velocity.x = 1.;
            velocity.y = 0.;
            *direction = Direction::Right;
        } else if kb.pressed(KeyCode::W) {
            velocity.y = 1.;
            velocity.x = 0.;
            *direction = Direction::Up;
        } else if kb.pressed(KeyCode::S) {
            velocity.y = -1.;
            velocity.x = 0.;
            *direction = Direction::Down;
        } else {
            velocity.x = 0.;
            velocity.y = 0.;
        };
    }
}
