use crate::{GameState, SIDE_WALK, WIN_HEIGHT, WIN_WIDTH};
use bevy::prelude::*;

const COLOR_RED: (f32, f32, f32) = (255. / 255., 89. / 255., 94. / 255.);
const COLOR_YELLOW: (f32, f32, f32) = (255. / 255., 202. / 255., 58. / 255.);

pub struct MenuPlugin;

#[derive(Component)]
pub struct Ancestor;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_ui_system))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(button_control_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn_ui_system));
    }
}

fn setup_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(WIN_WIDTH), Val::Px(WIN_HEIGHT + (SIDE_WALK * 2.))),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            background_color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2).into(),
            ..default()
        })
        .insert(Ancestor)
        .with_children(|parent| {
            // Top Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "The Chicken Road",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 100.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                });
            // End of Top Container

            // Middle Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(250.), Val::Px(65.)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::rgb(
                                COLOR_YELLOW.0,
                                COLOR_YELLOW.1,
                                COLOR_YELLOW.2,
                            )
                            .into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "START",
                                TextStyle {
                                    font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                                    font_size: 50.,
                                    color: Color::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2),
                                },
                            ));
                        });
                });
            // End of Middle Container

            // Bottom Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "COMMANDS",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 35.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Enter -> START",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "P -> PAUSE",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "W -> FORWARD",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "D -> RIGHT",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "S -> DOWNWARD",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "A -> LEFT",
                        TextStyle {
                            font: asset_server.load("fonts/RubikSprayPaint-Regular.ttf"),
                            font_size: 25.,
                            color: Color::rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2),
                        },
                    ));
                });

            // End of Bottom Container
        });
}

fn button_control_system(
    kb: Res<Input<KeyCode>>,
    mut button_query: Query<&Interaction, With<Button>>,
    mut game_state: ResMut<State<GameState>>,
    mut windows: ResMut<Windows>,
) {
    if kb.pressed(KeyCode::Return) {
        game_state.set(GameState::InGame).unwrap();
    }

    if let Ok(interaction) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::InGame).unwrap();
            }
            Interaction::Hovered => {
                let current_cursor = windows.primary().cursor_icon();
                if current_cursor != CursorIcon::Hand {
                    windows.primary_mut().set_cursor_icon(CursorIcon::Hand);
                }
            }
            _ => {}
        }
    }
}

fn despawn_ui_system(mut commands: Commands, mut node_query: Query<Entity, With<Ancestor>>) {
    if let Ok(ent) = node_query.get_single_mut() {
        commands.entity(ent).despawn_recursive();
    }
}
