use crate::{
    menu::{button_control_system, despawn_ui_system, Ancestor},
    player::PlayerState,
    GameAssets, GameState, COLOR_RED, COLOR_YELLOW, SIDE_WALK, WIN_HEIGHT, WIN_WIDTH,
};
use bevy::prelude::*;
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(setup_game_over_ui_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(button_control_system),
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(despawn_ui_system));
    }
}

fn setup_game_over_ui_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    player_state: Res<PlayerState>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(WIN_WIDTH), Val::Px(WIN_HEIGHT + (SIDE_WALK * 2.))),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            background_color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2).into(),
            ..default()
        })
        .insert(Ancestor)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 100.,
                    color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2),
                },
            ));
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("You reached the level {}", player_state.level),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 25.,
                    color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2),
                },
            ));
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(320.), Val::Px(65.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PLAY AGAIN",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 50.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                });
        });
}
