use crate::{GameAssets, BASE_SPEED, PLAYER_DIM, SIDE_WALK, TIME_STEP, WIN_HEIGHT, WIN_WIDTH};
use bevy::prelude::*;

const PLAYER_RIGHT_SPRITE_INDEX: (usize, usize) = (0, 5);
const PLAYER_UP_SPRITE_INDEX: (usize, usize) = (6, 11);
const PLAYER_LEFT_SPRITE_INDEX: (usize, usize) = (12, 17);
const PLAYER_DOWN_SPRITE_INDEX: (usize, usize) = (18, 23);

const ARENA_LEFT: f32 = -WIN_WIDTH / 2. + PLAYER_DIM;
const ARENA_RIGHT: f32 = WIN_WIDTH / 2. - PLAYER_DIM;
const ARENA_TOP: f32 = (WIN_HEIGHT + SIDE_WALK * 2.) / 2. - PLAYER_DIM;
const ARENA_BOTTOM: f32 = -(WIN_HEIGHT + SIDE_WALK * 2.) / 2. + PLAYER_DIM;

fn get_sprite_index(dim: (usize, usize), current_index: usize) -> usize {
    let index = if current_index >= dim.0 && current_index < dim.1 {
        current_index + 1
    } else {
        dim.0
    };
    return index;
}

#[derive(PartialEq, Eq)]
pub enum Sidewalk {
    Top,
    Bottom,
    None,
}

#[derive(Resource)]
pub struct PlayerState {
    pub alive: bool,
    pub level: u8,
    pub last_sidewalk: Sidewalk,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: true,
            level: 1,
            last_sidewalk: Sidewalk::None,
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, game_textures: Res<GameAssets>) {
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

pub fn move_player_system(
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
        if velocity.x != 0. || velocity.y != 0. {
            let translation = &mut transform.translation;

            let new_player_x_position = translation.x + velocity.x * TIME_STEP * BASE_SPEED;
            let new_player_y_position = translation.y + velocity.y * TIME_STEP * BASE_SPEED;

            translation.x = new_player_x_position.clamp(ARENA_LEFT, ARENA_RIGHT);
            translation.y = new_player_y_position.clamp(ARENA_BOTTOM, ARENA_TOP);

            sprite.index = match direction {
                Direction::Up => get_sprite_index(PLAYER_UP_SPRITE_INDEX, sprite.index),
                Direction::Down => get_sprite_index(PLAYER_DOWN_SPRITE_INDEX, sprite.index),
                Direction::Left => get_sprite_index(PLAYER_LEFT_SPRITE_INDEX, sprite.index),
                Direction::Right => get_sprite_index(PLAYER_RIGHT_SPRITE_INDEX, sprite.index),
            }
        }
    }
}

pub fn player_input_system(
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
